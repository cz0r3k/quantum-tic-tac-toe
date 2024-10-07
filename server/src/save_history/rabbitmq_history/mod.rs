#[cfg(test)]
mod test;

use crate::save_history::SaveHistory;
use crate::server_error::ServerError;
use async_trait::async_trait;
use error_stack::{report, Result, ResultExt};
use futures::StreamExt;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use ipc::rabbitmq::{CONSUMER_CLIENT, QUEUE_GET_GAME, QUEUE_SAVE_GAME};
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::{FieldTable, ShortString};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Queue};
use uuid::Uuid;

struct RabbitmqHistory {
    save_channel: Channel,
    game_channel: Channel,
    callback_queue: Queue,
    correlation_id: ShortString,
}

#[async_trait]
impl SaveHistory for RabbitmqHistory {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), ServerError> {
        let encode = bincode::serialize(&moves_history)
            .change_context(ServerError::SerializationError)
            .attach_printable("Can't be serialized")?;
        self.save_channel
            .basic_publish(
                "",
                QUEUE_SAVE_GAME,
                BasicPublishOptions::default(),
                &encode,
                BasicProperties::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't publish")?;
        Ok(())
    }

    #[allow(unused)]
    async fn get_game_history(&self, game_uuid: Uuid) -> Result<GameHistory, ServerError> {
        let mut consumer = self
            .game_channel
            .basic_consume(
                self.callback_queue.name().as_str(),
                CONSUMER_CLIENT,
                BasicConsumeOptions {
                    no_ack: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create consumer")?;

        let encode = bincode::serialize(&game_uuid)
            .change_context(ServerError::SerializationError)
            .attach_printable("Can't be serialized")?;
        self.game_channel
            .basic_publish(
                "",
                QUEUE_GET_GAME,
                BasicPublishOptions::default(),
                &encode,
                BasicProperties::default()
                    .with_reply_to(self.callback_queue.name().clone())
                    .with_correlation_id(self.correlation_id.clone()),
            )
            .await
            .change_context(ServerError::RabbitMQError)?
            .await
            .change_context(ServerError::RabbitMQError)?;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                if delivery.properties.correlation_id().as_ref() == Some(&self.correlation_id) {
                    let decode = bincode::deserialize::<GameHistory>(&delivery.data)
                        .change_context(ServerError::SerializationError)?;
                    return Ok(decode);
                }
            }
        }
        let err = report!(ServerError::RabbitMQError).attach_printable("No reply");
        Err(err)
    }
}

impl RabbitmqHistory {
    #[allow(unused)]
    pub async fn new(connection_string: &str) -> Result<Self, ServerError> {
        let connection = Connection::connect(connection_string, ConnectionProperties::default())
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't connect")?;

        let save_channel = connection
            .create_channel()
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create channel")?;
        save_channel
            .queue_declare(
                QUEUE_SAVE_GAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)?;

        let game_channel = connection
            .create_channel()
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create channel")?;
        let callback_queue = game_channel
            .queue_declare(
                "",
                QueueDeclareOptions {
                    exclusive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create callback queue")?;

        let correlation_id = Uuid::new_v4().to_string().into();

        Ok(Self {
            save_channel,
            game_channel,
            callback_queue,
            correlation_id,
        })
    }
}
