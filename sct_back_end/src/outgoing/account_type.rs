pub enum AccountType {
  Profile,
  Id,
}

impl AccountType {
  pub fn from_string(string: &str) -> AccountType {
    match string {
      "id" => AccountType::Id,
      // Profile is the prevalent type, so everything that is not "id" is considered a profile.
      _ => AccountType::Profile,
    }
  }
}

// Todo: Is this sensible?
impl Clone for AccountType {
  fn clone(&self) -> Self {
    match self {
      AccountType::Profile => AccountType::Profile,
      AccountType::Id => AccountType::Id,
    }
  }
}

impl Copy for AccountType {}
