use crate::game_repository::GameRepository;
use std::collections::HashSet;
use uuid::Uuid;

pub struct LocalRepository {
    repository: HashSet<Uuid>,
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
