use config::{Config, ConfigError, File};

use crate::config::server::Server;
use crate::config::version::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
  // Todo: The version is only for testing/debugging. It should be removed at some point.
  pub version: Version,
  pub server: Server,
}

impl Application {
  fn from_environment(prefix: &str) -> config::Environment {
    config::Environment::with_prefix(prefix)
      .separator("_")
      .keep_prefix(true)
  }

  pub fn load() -> Result<Self, ConfigError> {
    Config::builder()
      .add_source(File::with_name("config/application"))
      .add_source(Self::from_environment(Version::PREFIX))
      .add_source(Self::from_environment(Server::PREFIX))
      .build()?
      .try_deserialize()
  }
}
