#[cfg(test)]
mod test;

use crate::save_history::SaveHistory;
use crate::server_error::ServerError;
use async_trait::async_trait;
use error_stack::{report, Result, ResultExt};
use futures::StreamExt;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::{FieldTable, ShortString};
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Queue};
use uuid::Uuid;

const QUEUE_SAVE_GAME: &str = "game_history";
const QUEUE_GET_GAME: &str = "get_game_history";
const CONSUMER_TAG: &str = "rpc_client";

struct Rabbitmq {
    save_channel: Channel,
    game_channel: Channel,
    callback_queue: Queue,
    consumer: Consumer,
    correlation_id: ShortString,
}

#[async_trait]
impl SaveHistory for Rabbitmq {
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
    async fn get_game_history(&mut self, game_uuid: Uuid) -> Result<GameHistory, ServerError> {
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

        while let Some(delivery) = self.consumer.next().await {
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

impl Rabbitmq {
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
        let consumer = game_channel
            .basic_consume(
                callback_queue.name().as_str(),
                CONSUMER_TAG,
                BasicConsumeOptions {
                    no_ack: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create consumer")?;

        let correlation_id = Uuid::new_v4().to_string().into();

        Ok(Self {
            save_channel,
            game_channel,
            callback_queue,
            consumer,
            correlation_id,
        })
    }
}
