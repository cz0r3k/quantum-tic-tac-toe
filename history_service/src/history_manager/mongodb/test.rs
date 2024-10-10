use crate::history_manager::mongodb::MongodbHistory;
use crate::history_manager::HistoryManager;
use ipc::game_history::GameHistory;
use ipc::moves_history::MovesHistory;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;
use uuid::{uuid, Uuid};

const UUID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
const MONGODB_PORT: u16 = 27017;
const ADDRESS: &str = "mongodb://127.0.0.1";
const IMAGE_NAME: &str = "mongo";
const IMAGE_TAG: &str = "8.0";

#[tokio::test]
async fn insert_empty_game_history() {
    let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
        .with_exposed_port(MONGODB_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
        .start()
        .await
        .expect("Container should start");

    let ports = container.ports().await.unwrap();
    let port = ports.map_to_host_port_ipv4(MONGODB_PORT.tcp()).unwrap();
    let mongodb_connection_string = format!("{ADDRESS}:{port}");
    let move_history = MovesHistory::new(UUID, 3);
    let history_manager = MongodbHistory::new(&mongodb_connection_string)
        .await
        .unwrap();
    assert!(history_manager.save_game(&move_history).await.is_ok());
}

#[tokio::test]
async fn get_empty_game_history() {
    let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
        .with_exposed_port(MONGODB_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Waiting for connections"))
        .start()
        .await
        .expect("Container should start");

    let ports = container.ports().await.unwrap();
    let port = ports.map_to_host_port_ipv4(MONGODB_PORT.tcp()).unwrap();
    let mongodb_connection_string = format!("{ADDRESS}:{port}");
    let move_history = MovesHistory::new(UUID, 3);
    let history_manager = MongodbHistory::new(&mongodb_connection_string)
        .await
        .unwrap();
    history_manager.save_game(&move_history).await.unwrap();
    let game_history = GameHistory::try_from(&move_history).unwrap();
    let new_game_history = history_manager.get_game_history(UUID).await.unwrap();
    assert_eq!(new_game_history, game_history);
}
