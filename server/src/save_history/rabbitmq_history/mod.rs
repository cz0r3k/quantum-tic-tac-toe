#[cfg(test)]
mod test;

use crate::save_history::SaveHistory;
use crate::server_error::ServerError;
use async_trait::async_trait;
use error_stack::{Result, ResultExt};
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use uuid::Uuid;

const QUEUE_NAME: &str = "game_history";

struct Rabbitmq {
    channel: Channel,
}

#[async_trait]
impl SaveHistory for Rabbitmq {
    async fn save_game(&self, moves_history: &MovesHistory) -> Result<(), ServerError> {
        let encode = bincode::serialize(&moves_history)
            .change_context(ServerError::SerializationError)
            .attach_printable("Can't be serialized")?;
        self.channel
            .basic_publish(
                "",
                QUEUE_NAME,
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
        unimplemented!()
    }
}

impl Rabbitmq {
    #[allow(unused)]
    pub async fn new(connection_string: &str) -> Result<Self, ServerError> {
        let connection = Connection::connect(connection_string, ConnectionProperties::default())
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't connect")?;
        let channel = connection
            .create_channel()
            .await
            .change_context(ServerError::RabbitMQError)
            .attach_printable("Can't create channel")?;
        channel
            .queue_declare(
                QUEUE_NAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .change_context(ServerError::RabbitMQError)?;
        Ok(Self { channel })
    }
}
