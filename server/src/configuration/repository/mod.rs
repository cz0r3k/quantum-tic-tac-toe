#[cfg(test)]
mod test;

use clap::ValueEnum;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Repository {
    Local,
    Redis,
}

impl FromStr for Repository {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "LOCAL" => Ok(Repository::Local),
            "REDIS" => Ok(Repository::Redis),
            _ => Err(()),
        }
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Repository::Local => f.write_str("LOCAL"),
            Repository::Redis => f.write_str("REDIS"),
        }
    }
}
