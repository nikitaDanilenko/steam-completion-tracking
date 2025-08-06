use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_error::SteamError;
use reqwest::header::COOKIE;

fn endpoint_by(account_type: &AccountType, account_id: &str) -> String {
  let identifier = match account_type {
    AccountType::Profile => "profiles",
    AccountType::Id => "id",
  };

  let address = format!(
    "https://steamcommunity.com/{}/{}/games/?tab=all",
    identifier, account_id
  );
  address
}

pub async fn call_games_endpoint(
  account_type: &AccountType,
  account_id: &str,
  steam_token: &str,
) -> Result<String, SteamError> {
  let url = endpoint_by(account_type, account_id);
  let client = reqwest::Client::new();
  let response = client
    .get(&url)
    // Todo: This may be unnecessary, because the cookie already needs to be taken from the existing cookies.
    // It may be better to pass all cookies or at least avoid the reconstruction of the cookie name.
    .header(COOKIE, format!("steamLoginSecure={}", steam_token))
    .send()
    .await;

  match response {
    // Todo: Handle better, this seems really haphazard
    Ok(result) => Ok(
      result
        .text()
        .await
        // Todo: Handle better, this seems really haphazard
        .unwrap_or_else(|_| String::from("error")),
    ),
    Err(e) => Err(SteamError {
      error: e.to_string(),
    }),
  }
}
