mod repository;

use crate::configuration::repository::Repository;
use crate::game_repository::local_repository::LocalRepository;
use crate::game_repository::redis_repository::RedisRepository;
use crate::game_repository::{GameRepository, GameRepositoryEnum};
use crate::server_error::ServerError;
use clap::Parser;
use error_stack::Result;
use log::{info, LevelFilter};
use std::env;
use std::net::IpAddr::V4;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;

const DEFAULT_SERVER_ADDRESS: IpAddr = V4(Ipv4Addr::LOCALHOST);
const DEFAULT_SERVER_PORT: u16 = 6379;
const DEFAULT_GAME_REPOSITORY: Repository = Repository::Local;
const DEFAULT_REDIS_CONNECTION_STRING: &str = "redis://127.0.0.1/";

#[derive(Debug)]
pub struct Configuration {
    server_address: SocketAddr,
    game_repository: GameRepositoryEnum,
    debug: bool,
}

impl Configuration {
    pub fn new() -> Self {
        let cli = Cli::parse();

        Self {
            server_address: SocketAddr::new(Self::get_address(&cli), Self::get_port(&cli)),
            game_repository: Self::get_game_repository(&cli),
            debug: cli.debug,
        }
    }
    pub fn server_address(&self) -> &SocketAddr {
        &self.server_address
    }
    pub fn game_repository(&self) -> &GameRepositoryEnum {
        &self.game_repository
    }

    pub fn set_env_logger(&self) {
        if self.debug {
            env_logger::builder()
                .filter_level(LevelFilter::Debug)
                .init();
        } else {
            env_logger::init();
        }
    }

    pub async fn create_game_repository(
        &self,
    ) -> Result<Arc<Mutex<Box<dyn GameRepository>>>, ServerError> {
        let game_repository: Arc<Mutex<Box<dyn GameRepository>>> =
            Arc::new(Mutex::new(match self.game_repository() {
                GameRepositoryEnum::Redis(connection_string) => {
                    let mut repository = RedisRepository::new(connection_string);
                    info!("Connecting to redis: {connection_string}");
                    repository.connect().await?;
                    info!("Connected to redis");
                    Box::new(repository)
                }
                GameRepositoryEnum::Local => Box::new(LocalRepository::new()),
            }));
        Ok(game_repository)
    }

    fn get_address(cli: &Cli) -> IpAddr {
        if let Some(address) = cli.address {
            address
        } else if let Ok(address) = env::var("ADDRESS") {
            if let Ok(address) = address.parse() {
                V4(address)
            } else {
                eprintln!("error parsing ADDRESS environment variable ({address})");
                eprintln!("ADDRESS is set to {DEFAULT_SERVER_ADDRESS}");
                DEFAULT_SERVER_ADDRESS
            }
        } else {
            DEFAULT_SERVER_ADDRESS
        }
    }

    fn get_port(cli: &Cli) -> u16 {
        if let Some(port) = cli.port {
            port
        } else if let Ok(port) = env::var("PORT") {
            if let Ok(port) = port.parse() {
                port
            } else {
                eprintln!("error parsing PORT environment variable ({port})");
                eprintln!("PORT is set to {DEFAULT_SERVER_PORT}");
                DEFAULT_SERVER_PORT
            }
        } else {
            DEFAULT_SERVER_PORT
        }
    }

    fn get_repository(cli: &Cli) -> Repository {
        if let Some(repository) = cli.game_repository {
            repository
        } else if let Ok(repository) = env::var("GAME_REPOSITORY") {
            if let Ok(repository) = repository.parse() {
                repository
            } else {
                eprintln!("error parsing GAME_REPOSITORY environment variable ({repository})");
                eprintln!("GAME_REPOSITORY is set to {DEFAULT_GAME_REPOSITORY}");
                DEFAULT_GAME_REPOSITORY
            }
        } else {
            DEFAULT_GAME_REPOSITORY
        }
    }

    fn get_game_repository(cli: &Cli) -> GameRepositoryEnum {
        match Self::get_repository(cli) {
            Repository::Local => GameRepositoryEnum::Local,
            Repository::Redis => {
                GameRepositoryEnum::Redis(if let Some(redis_string) = &cli.redis_connection_string {
                    redis_string.to_string()
                } else if let Ok(redis_string) = env::var("REDIS_CONNECTION_STRING") {
                    redis_string
                } else {
                    DEFAULT_REDIS_CONNECTION_STRING.to_string()
                })
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    address: Option<IpAddr>,
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,
    #[arg(long, value_enum, ignore_case = true)]
    game_repository: Option<Repository>,
    #[arg(long)]
    redis_connection_string: Option<String>,
    /// Debug mode
    #[arg(short, default_value_t = false)]
    debug: bool,
}
