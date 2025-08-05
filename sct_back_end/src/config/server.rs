use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub port: u16,
}

impl Server {
  pub const PREFIX: &'static str = "SERVER";
}
