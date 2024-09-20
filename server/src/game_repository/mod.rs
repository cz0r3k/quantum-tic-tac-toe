pub mod local_repository;
pub mod redis_repository;

use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait GameRepository: Send {
    async fn add_game(&mut self, uuid: Uuid) -> bool;
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum GameRepositoryEnum {
    Redis(String),
    Local,
}
