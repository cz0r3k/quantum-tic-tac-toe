use crate::game_repository::GameRepository;
use crate::server_error::ServerError;
use error_stack::ResultExt;
use redis::AsyncCommands;
use uuid::Uuid;

pub struct RedisRepository {
    connection_string: String,
    connection: Option<redis::aio::MultiplexedConnection>,
}

impl RedisRepository {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            connection: None,
        }
    }
    pub async fn connect(&mut self) -> error_stack::Result<(), ServerError> {
        let client = redis::Client::open(self.connection_string.clone())
            .change_context(ServerError::RedisError)
            .attach_printable("Can't create redis client")?;
        self.connection = Some(
            client
                .get_multiplexed_tokio_connection()
                .await
                .change_context(ServerError::RedisError)
                .attach_printable("Can't create redis connection")?,
        );
        Ok(())
    }
}

impl GameRepository for RedisRepository {
    async fn add_game(&mut self, uuid: Uuid) -> bool {
        match self.connection {
            Some(ref mut connection) => match connection.sadd("games", uuid).await {
                Ok(result) => matches!(result, 1),
                Err(_) => false,
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use testcontainers::{
        core::{IntoContainerPort, WaitFor},
        runners::AsyncRunner,
        GenericImage,
    };
    use tokio::sync::Mutex;
    use uuid::{uuid, Uuid};
    const UUID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    const REDIS_PORT: u16 = 6379;
    const ADDRESS: &str = "redis://127.0.0.1";
    const IMAGE_NAME: &str = "valkey/valkey";
    const IMAGE_TAG: &str = "8.0";

    #[tokio::test]
    async fn add_one_game() {
        let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
            .with_exposed_port(REDIS_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .expect("Container should start");
        let ports = container.ports().await.unwrap();
        let port = ports.map_to_host_port_ipv4(REDIS_PORT.tcp()).unwrap();
        let redis_connection_string = format!("{ADDRESS}:{port}/",);
        let game_repository_redis =
            Arc::new(Mutex::new(RedisRepository::new(&redis_connection_string)));
        game_repository_redis.lock().await.connect().await.unwrap();
        assert!(game_repository_redis.lock().await.add_game(UUID).await);
    }

    #[tokio::test]
    async fn add_same_uuid() {
        let container = GenericImage::new(IMAGE_NAME, IMAGE_TAG)
            .with_exposed_port(REDIS_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .expect("Container should start");
        let ports = container.ports().await.unwrap();
        let port = ports.map_to_host_port_ipv4(REDIS_PORT.tcp()).unwrap();
        let redis_connection_string = format!("{ADDRESS}:{port}/");
        let game_repository_redis =
            Arc::new(Mutex::new(RedisRepository::new(&redis_connection_string)));
        game_repository_redis.lock().await.connect().await.unwrap();
        assert!(game_repository_redis.lock().await.add_game(UUID).await);
        assert!(!game_repository_redis.lock().await.add_game(UUID).await);
    }
}
