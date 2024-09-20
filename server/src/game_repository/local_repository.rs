use crate::game_repository::GameRepository;
use async_trait::async_trait;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct LocalRepository {
    repository: HashSet<Uuid>,
}

impl LocalRepository {
    pub fn new() -> Self {
        Self {
            repository: HashSet::new(),
        }
    }
}

#[async_trait]
impl GameRepository for LocalRepository {
    async fn add_game(&mut self, uuid: Uuid) -> bool {
        if self.repository.contains(&uuid) {
            false
        } else {
            self.repository.insert(uuid);
            true
        }
    }
}
