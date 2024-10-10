pub mod rabbitmq_handler;

use crate::history_manager::HistoryManager;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
#[allow(unused)]
pub trait MessageHandler {
    async fn save_game<MANAGER: HistoryManager + Send + Sync>(&self, history_manager: Arc<MANAGER>);
    async fn get_game<MANAGER: HistoryManager + Send + Sync>(&self, history_manager: Arc<MANAGER>);
}
