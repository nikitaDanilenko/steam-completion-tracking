use config::{Config, ConfigError, Environment, File};

use crate::config::server::Server;
use crate::config::version::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
  pub version: Version,
  pub server: Server,
}

impl Application {
  pub fn load() -> Result<Self, ConfigError> {
    Config::builder()
      .add_source(File::with_name("config/application"))
      .add_source(Environment::with_prefix("SERVER").separator("_"))
      .build()?
      .try_deserialize()
  }
}
