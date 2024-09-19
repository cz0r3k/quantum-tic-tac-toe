mod local_repository;
pub mod redis_repository;

use uuid::Uuid;

pub trait GameRepository {
    async fn add_game(&mut self, uuid: Uuid) -> bool;
}

#[allow(unused, clippy::module_name_repetitions)]
pub enum GameRepositoryEnum {
    Redis(String),
    Local,
}
