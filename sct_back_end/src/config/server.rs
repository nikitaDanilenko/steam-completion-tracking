use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub port: u16,
}
