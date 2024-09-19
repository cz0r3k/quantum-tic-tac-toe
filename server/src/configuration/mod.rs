use crate::game_repository::GameRepositoryEnum;
use std::net::SocketAddr;

#[allow(unused)]
pub struct Configuration {
    server_address: SocketAddr,
    game_repository: GameRepositoryEnum,
}
