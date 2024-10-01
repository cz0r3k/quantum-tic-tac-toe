use super::*;
use futures::StreamExt;
use lapin::options::BasicConsumeOptions;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use std::sync::Arc;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};
use tokio::sync::Barrier;
use uuid::uuid;

const UUID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
const RABBITMQ_PORT: u16 = 5672;
const ADDRESS: &str = "amqp://127.0.0.1";
const IMAGE_NAME: &str = "rabbitmq";
const IMAGE_TAG: &str = "4.0";

async fn consume_message(connection_string: &str) -> Vec<u8> {
    let conn = Connection::connect(connection_string, ConnectionProperties::default())
        .await
        .unwrap();
    let channel = conn.create_channel().await.unwrap();

    channel
        .queue_declare(
            QUEUE_SAVE_GAME,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let mut consumer = channel
        .basic_consume(
            QUEUE_SAVE_GAME,
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let delivery = consumer.next().await.unwrap().unwrap();
    delivery.ack(BasicAckOptions::default()).await.unwrap();
    delivery.data
}

async fn handle_get_history(connection_string: &str, barrier: Arc<Barrier>) {
    let conn = Connection::connect(connection_string, ConnectionProperties::default())
        .await
        .unwrap();
    let channel = conn.create_channel().await.unwrap();
    channel
        .queue_declare(
            QUEUE_GET_GAME,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();
    channel
        .basic_qos(1, BasicQosOptions::default())
        .await
        .unwrap();
    let mut consumer = channel
        .basic_consume(
            QUEUE_GET_GAME,
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    barrier.wait().await;

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let uuid: Uuid = bincode::deserialize(&delivery.data).unwrap();

            let moves_history = MovesHistory::new(uuid, 3);
            let game_history = GameHistory::try_from(&moves_history).unwrap();
            let encode = bincode::serialize(&game_history).unwrap();

            let routing_key = delivery.properties.reply_to().as_ref().unwrap().as_str();
            let correlation_id = delivery.properties.correlation_id().clone().unwrap();

            channel
                .basic_publish(
                    "",
                    routing_key,
                    BasicPublishOptions::default(),
                    &encode,
                    BasicProperties::default().with_correlation_id(correlation_id),
                )
                .await
                .unwrap();
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await
                .unwrap();
            break;
        }
    }
}

#[tokio::test]
async fn send_empty_move_history() {
    let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
        .with_exposed_port(RABBITMQ_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stdout(
            "Ready to start client connection listeners",
        ))
        .start()
        .await
        .expect("Container should start");
    let ports = container.ports().await.unwrap();
    let port = ports.map_to_host_port_ipv4(RABBITMQ_PORT.tcp()).unwrap();
    let rabbitmq_connection_string = format!("{ADDRESS}:{port}");

    let rabbitmq_history = Rabbitmq::new(&rabbitmq_connection_string).await.unwrap();
    let move_history = MovesHistory::new(UUID, 3);
    rabbitmq_history.save_game(&move_history).await.unwrap();

    let body = consume_message(&rabbitmq_connection_string).await;
    let move_history_new = bincode::deserialize::<MovesHistory>(&body).unwrap();
    assert_eq!(move_history, move_history_new);
}

#[tokio::test]
async fn get_empty_game_history() {
    let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
        .with_exposed_port(RABBITMQ_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stdout(
            "Ready to start client connection listeners",
        ))
        .start()
        .await
        .expect("Container should start");
    let ports = container.ports().await.unwrap();
    let port = ports.map_to_host_port_ipv4(RABBITMQ_PORT.tcp()).unwrap();
    let rabbitmq_connection_string = format!("{ADDRESS}:{port}");
    let barrier = Arc::new(Barrier::new(2));

    let mut rabbitmq_history = Rabbitmq::new(&rabbitmq_connection_string).await.unwrap();

    let barrier_clone = barrier.clone();
    let handle = tokio::spawn(async move {
        handle_get_history(&rabbitmq_connection_string, barrier_clone).await;
    });

    let move_history = MovesHistory::new(UUID, 3);
    let game_history = GameHistory::try_from(&move_history).unwrap();

    barrier.wait().await;
    let new_game_history = rabbitmq_history.get_game_history(UUID).await.unwrap();

    assert_eq!(new_game_history, game_history);
    handle.await.unwrap();
}
