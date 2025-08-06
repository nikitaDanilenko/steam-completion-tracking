/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///`Game`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "appId",
    ///    "name",
    ///    "totalAchievements",
    ///    "unlockedAchievements"
    ///  ],
    ///  "properties": {
    ///    "appId": {
    ///      "description": "The unique identifier for the game (from Steam).",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "description": "The name of the game.",
    ///      "type": "string"
    ///    },
    ///    "totalAchievements": {
    ///      "description": "The total number of achievements available in the game.",
    ///      "type": "integer"
    ///    },
    ///    "unlockedAchievements": {
    ///      "description": "The number of achievements unlocked in the game.",
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Game {
        ///The unique identifier for the game (from Steam).
        #[serde(rename = "appId")]
        pub app_id: i64,
        ///The name of the game.
        pub name: ::std::string::String,
        ///The total number of achievements available in the game.
        #[serde(rename = "totalAchievements")]
        pub total_achievements: i64,
        ///The number of achievements unlocked in the game.
        #[serde(rename = "unlockedAchievements")]
        pub unlocked_achievements: i64,
    }
    impl ::std::convert::From<&Game> for Game {
        fn from(value: &Game) -> Self {
            value.clone()
        }
    }
    ///`Stats`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "games",
    ///    "profileName",
    ///    "steamId"
    ///  ],
    ///  "properties": {
    ///    "games": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Game"
    ///      }
    ///    },
    ///    "profileName": {
    ///      "description": "The name of the Steam profile.",
    ///      "type": "string"
    ///    },
    ///    "steamId": {
    ///      "description": "The unique identifier for the Steam profile.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Stats {
        pub games: ::std::vec::Vec<Game>,
        ///The name of the Steam profile.
        #[serde(rename = "profileName")]
        pub profile_name: ::std::string::String,
        ///The unique identifier for the Steam profile.
        #[serde(rename = "steamId")]
        pub steam_id: ::std::string::String,
    }
    impl ::std::convert::From<&Stats> for Stats {
        fn from(value: &Stats) -> Self {
            value.clone()
        }
    }
}
