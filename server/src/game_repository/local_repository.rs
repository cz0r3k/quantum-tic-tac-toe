use crate::game_repository::GameRepository;
use std::collections::HashSet;
use uuid::Uuid;

pub struct LocalRepository {
    repository: HashSet<Uuid>,
}

impl LocalRepository {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            repository: HashSet::new(),
        }
    }
}

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
