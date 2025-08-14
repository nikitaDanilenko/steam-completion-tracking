use crate::models::steam::generated::types::AccountInformation;
use crate::outgoing::account_type::AccountType;
use crate::outgoing::steam_error::SteamError;
use reqwest::header::COOKIE;
use scraper::{Html, Selector};

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
) -> Result<AccountInformation, SteamError> {
  let url = endpoint_by(account_type, account_id);
  let client = reqwest::Client::new();
  let response = client
    .get(&url)
    // Todo: This may be unnecessary, because the cookie already needs to be taken from the existing cookies.
    // It may be better to pass all cookies or at least avoid the reconstruction of the cookie name.
    .header(COOKIE, format!("steamLoginSecure={}", steam_token))
    .send()
    .await
    .map_err(|error| SteamError {
      error: format!("Request failed: {}", error),
    })?;

  let html = response.text().await.map_err(|e| SteamError {
    error: format!("Failed to parse response: {}", e),
  })?;

  extract_account_information(&html)
}

const GAMES_SELECTOR: &str = "#gameslist_config";
const GAMES_ATTRIBUTE: &str = "data-profile-gameslist";

fn extract_account_information(html: &str) -> Result<AccountInformation, SteamError> {
  // Parse the HTML document
  let document = Html::parse_document(html);

  // Create a selector for the element with id "gameslist_config".
  // The element (template) with this id contains an attribute with the list of games.
  let selector = Selector::parse(GAMES_SELECTOR).unwrap();

  // Find the element and extract the attribute mentioned above
  document
    .select(&selector)
    .next()
    .ok_or(SteamError::from_string(format!(
      "No '{}' element found",
      GAMES_SELECTOR
    )))
    .and_then(|element| {
      element
        .value()
        .attr(GAMES_ATTRIBUTE)
        .ok_or(SteamError::from_string(format!(
          "No '{}' attribute found",
          GAMES_ATTRIBUTE
        )))
    })
    .and_then(|data| {
      let json = data.replace("&quot;", "\"").to_string();
      serde_json::from_str(&json).map_err(|e| SteamError {
        error: format!("Failed to parse JSON: {}", e),
      })
    })
}
