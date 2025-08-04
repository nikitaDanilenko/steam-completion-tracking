use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
  pub major: String,
  pub minor: String,
  pub patch: String,
}
