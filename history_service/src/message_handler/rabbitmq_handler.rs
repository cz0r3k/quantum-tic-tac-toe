use crate::history_error::HistoryError;
use crate::history_manager::HistoryManager;
use crate::message_handler::MessageHandler;
use async_trait::async_trait;
use error_stack::ResultExt;
use futures::StreamExt;
use ipc::moves_history::MovesHistory;
use ipc::rabbitmq::{CONSUMER_SERVER, QUEUE_GET_GAME, QUEUE_SAVE_GAME};
use lapin::options::{
    BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, BasicQosOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use std::sync::Arc;
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
        game_channel
            .basic_qos(1, BasicQosOptions::default())
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
    async fn save_game<MANAGER: HistoryManager + Send + Sync>(
        &self,
        history_manager: Arc<MANAGER>,
    ) {
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
                let move_history = match bincode::deserialize::<MovesHistory>(&delivery.data)
                    .change_context(HistoryError::DeserializeError)
                {
                    Ok(move_history) => move_history,
                    Err(_err) => continue,
                };
                match history_manager.save_game(&move_history).await {
                    Ok(()) => (),
                    Err(_err) => return,
                }
            }
        }
    }

    async fn get_game<MANAGER: HistoryManager + Send + Sync>(&self, history_manager: Arc<MANAGER>) {
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
                let uuid = bincode::deserialize::<Uuid>(&delivery.data)
                    .change_context(HistoryError::DeserializeError)
                    .unwrap();
                let game_history = history_manager.get_game_history(uuid).await.unwrap();
                let encode = bincode::serialize(&game_history)
                    .change_context(HistoryError::SerializeError)
                    .unwrap();
                let routing_key = delivery.properties.reply_to().as_ref().unwrap().as_str();
                let correlation_id = delivery.properties.correlation_id().clone().unwrap();
                self.game_channel
                    .basic_publish(
                        "",
                        routing_key,
                        BasicPublishOptions::default(),
                        &encode,
                        BasicProperties::default().with_correlation_id(correlation_id),
                    )
                    .await
                    .change_context(HistoryError::RabbitMQError)
                    .unwrap();
                self.game_channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .change_context(HistoryError::RabbitMQError)
                    .attach_printable("Game channel can't send ack")
                    .unwrap();
            }
        }
    }
}
