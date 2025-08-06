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
