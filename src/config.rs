//! Configuration module for the application.
//!
//! Configuration consists of two parts: `Bootstrap` and `Runtime`.
//!
//! `Bootstrap` is loaded at application startup, and is not expected to change during whole application lifecycle.
//! It is generally loaded from environment variables.
//! Mostly, Bootstrap should contain as little configuration as possible. In best practices, it only contains the necessary info to connect to the runtime configuration source so that
//! `Runtime` can be loaded.(e.g. DATABASE_URL, or credentials and endpoint to etcd, consul, vault, etc.)
//!
//! `Runtime` is loaded after `Bootstrap`, which should provide necessary info to fetch `Runtime` configuration from either a configuration distribution system, a remote config file, database, etc.
//!
//! **NOTE:** In this template, we load `Runtime` configs from database, to simplify the case.
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub bootstrap: Bootstrap,
    #[serde(flatten, default)]
    pub runtime: Runtime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Bootstrap {
    #[serde(default = "serde_defaults::default_listen")]
    pub listen: std::net::SocketAddr,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Runtime {}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::from_env().context(LoadEnvConfigSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("load configuration: {}", source))]
    LoadEnvConfig { source: envy::Error },
}

type Result<T> = std::result::Result<T, Error>;

mod serde_defaults {
    use std::net::{Ipv4Addr, SocketAddr};

    pub const fn default_listen() -> std::net::SocketAddr {
        SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)
    }
}
