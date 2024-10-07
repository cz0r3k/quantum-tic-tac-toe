pub mod rabbitmq_handler;

use async_trait::async_trait;

#[async_trait]
#[allow(unused)]
pub trait MessageHandler {
    async fn save_game(&self);
    async fn get_game(&self);
}
