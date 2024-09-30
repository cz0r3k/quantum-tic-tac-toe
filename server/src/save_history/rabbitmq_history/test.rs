use super::*;
use futures::StreamExt;
use lapin::options::BasicConsumeOptions;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage,
};
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

#[tokio::test]
async fn send_empty_move_history() {
    let move_history = MovesHistory::new(UUID, 3);
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
    rabbitmq_history.save_game(&move_history).await.unwrap();

    let body = consume_message(&rabbitmq_connection_string).await;
    let move_history_new = bincode::deserialize::<MovesHistory>(&body).unwrap();
    assert_eq!(move_history, move_history_new);
}
