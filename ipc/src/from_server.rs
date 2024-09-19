use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    Test,
    GameCreated(Uuid),
}
