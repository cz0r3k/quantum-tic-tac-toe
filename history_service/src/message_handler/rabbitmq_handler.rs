use crate::message_handler::MessageHandler;
use async_trait::async_trait;
use futures::StreamExt;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use ipc::rabbitmq::{CONSUMER_SERVER, QUEUE_GET_GAME, QUEUE_SAVE_GAME};
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties};
use uuid::Uuid;

#[allow(unused)]
struct RabbitMQHandler {
    save_channel: Channel,
    game_channel: Channel,
}

#[allow(unused)]
impl RabbitMQHandler {
    pub async fn new(connection_string: &str) -> Result<Self, ()> {
        let connection = Connection::connect(connection_string, ConnectionProperties::default())
            .await
            .unwrap();
        let save_channel = connection.create_channel().await.unwrap();
        save_channel
            .queue_declare(
                QUEUE_SAVE_GAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        let game_channel = connection.create_channel().await.unwrap();
        game_channel
            .queue_declare(
                QUEUE_GET_GAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        Ok(Self {
            save_channel,
            game_channel,
        })
    }
}

#[async_trait]
impl MessageHandler for RabbitMQHandler {
    async fn save_game(&self) {
        let mut consumer = self
            .save_channel
            .basic_consume(
                QUEUE_SAVE_GAME,
                CONSUMER_SERVER,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let move_history = bincode::deserialize::<MovesHistory>(&delivery.data).unwrap();
                let _game_history = GameHistory::try_from(&move_history).unwrap();
                todo!()
            }
        }
        todo!()
    }

    async fn get_game(&self) {
        let mut consumer = self
            .save_channel
            .basic_consume(
                QUEUE_SAVE_GAME,
                CONSUMER_SERVER,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let _uuid = bincode::deserialize::<Uuid>(&delivery.data).unwrap();
                todo!()
            }
        }
        todo!()
    }
}
