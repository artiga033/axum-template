use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

macro_rules! config_type {
    ($(
        $item:item
    )*) => {
        $(
            #[derive(Debug, Serialize, Deserialize)]
            $item
        )*
    };
}

config_type! {
    pub struct Config {
        #[serde(flatten)]
        pub bootstrap: Bootstrap,
        #[serde(flatten,default)]
        pub runtime: Runtime,
    }
    pub struct Bootstrap {
        #[serde(default = "serde_defaults::default_listen")]
        pub listen: std::net::SocketAddr,
    }
    #[derive(Default)]
    pub struct Runtime {
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::from_env().context(LoadConfigSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum ConfigError {
    #[snafu(display("Failed to load configuration: {}", source))]
    LoadConfig { source: envy::Error },
}

type Result<T> = std::result::Result<T, ConfigError>;

mod serde_defaults {
    use std::net::{Ipv4Addr, SocketAddr};

    pub const fn default_listen() -> std::net::SocketAddr {
        SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)
    }
}
