use super::*;
use crate::game_repository::local_repository::LocalRepository;
use ipc::from_server::board_ipc::Board;
use ipc::from_server::FromServer;
use ipc::game_configuration::GameConfiguration;
use uuid::Uuid;

#[tokio::test]
async fn create_new_game() {
    let game_configuration = GameConfiguration::default();
    let repository = Arc::new(Mutex::new(Box::new(LocalRepository::new())));
    let reader = tokio_test::io::Builder::new()
        .read(&bincode::serialize(&ToServer::CreateGame(game_configuration)).unwrap())
        .build();
    let writer = tokio_test::io::Builder::new()
        .write(&bincode::serialize(&FromServer::GameCreated(Uuid::nil())).unwrap())
        .write(&bincode::serialize(&FromServer::Board(Board::default())).unwrap())
        .build();
    let () = process(reader, writer, repository).await;
}

#[tokio::test]
async fn create_game_two_times() {
    let game_configuration = GameConfiguration::default();
    let repository = Arc::new(Mutex::new(Box::new(LocalRepository::new())));
    let reader = tokio_test::io::Builder::new()
        .read(&bincode::serialize(&ToServer::CreateGame(game_configuration)).unwrap())
        .read(&bincode::serialize(&ToServer::CreateGame(game_configuration)).unwrap())
        .build();
    let writer = tokio_test::io::Builder::new()
        .write(&bincode::serialize(&FromServer::GameCreated(Uuid::nil())).unwrap())
        .write(&bincode::serialize(&FromServer::Board(Board::default())).unwrap())
        .write(&bincode::serialize(&FromServer::GameAlreadyCreated).unwrap())
        .build();
    let () = process(reader, writer, repository).await;
}

#[tokio::test]
async fn ping() {
    let repository = Arc::new(Mutex::new(Box::new(LocalRepository::new())));
    let reader = tokio_test::io::Builder::new()
        .read(&bincode::serialize(&ToServer::PING).unwrap())
        .build();
    let writer = tokio_test::io::Builder::new()
        .write(&bincode::serialize(&FromServer::PONG).unwrap())
        .build();
    let () = process(reader, writer, repository).await;
}
