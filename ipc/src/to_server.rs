use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ToServer {
    CreateGame,
    Test,
    EndConnection,
}
