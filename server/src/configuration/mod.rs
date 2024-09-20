use crate::game_repository::GameRepositoryEnum;
use clap::builder::ArgPredicate;
use clap::{Parser, ValueEnum};
use std::net::IpAddr::V4;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
        let server_address = SocketAddr::new(cli.address, cli.port);
        let game_repository = match cli.game_repository {
            Repository::Local => GameRepositoryEnum::Local,
            Repository::Redis => GameRepositoryEnum::Redis(cli.redis_connection_string.unwrap()),
        };
        let debug = cli.debug;
        Self {
            server_address,
            game_repository,
            debug,
        }
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn server_address(&self) -> &SocketAddr {
        &self.server_address
    }
    pub fn game_repository(&self) -> &GameRepositoryEnum {
        &self.game_repository
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Repository {
    Local,
    Redis,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = DEFAULT_SERVER_ADDRESS)]
    address: IpAddr,
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..),
        default_value_t = DEFAULT_SERVER_PORT)]
    port: u16,
    #[arg(long, value_enum, default_value_t = DEFAULT_GAME_REPOSITORY, ignore_case = true)]
    game_repository: Repository,
    #[arg(
        long,
        default_value_if(
            "game_repository",
            ArgPredicate::IsPresent,
            Some(DEFAULT_REDIS_CONNECTION_STRING)
        )
    )]
    redis_connection_string: Option<String>,
    #[arg(short, default_value_t = false)]
    debug: bool,
}
