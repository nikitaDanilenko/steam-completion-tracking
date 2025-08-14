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
    /**Steam's representation of achievement progress.
There are more fields in the response, but two are redundant ('percentage' and 'all_unlocked'),
while 'cache_time' is not relevant for the achievement progress.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Steam's representation of achievement progress.\nThere are more fields in the response, but two are redundant ('percentage' and 'all_unlocked'),\nwhile 'cache_time' is not relevant for the achievement progress.\n",
    ///  "type": "object",
    ///  "required": [
    ///    "appid",
    ///    "total",
    ///    "unlocked",
    ///    "vetted"
    ///  ],
    ///  "properties": {
    ///    "appid": {
    ///      "description": "The unique identifier for the game.",
    ///      "type": "integer"
    ///    },
    ///    "total": {
    ///      "description": "The total number of achievements available in the game.",
    ///      "type": "integer"
    ///    },
    ///    "unlocked": {
    ///      "description": "The number of achievements unlocked in the game.",
    ///      "type": "integer"
    ///    },
    ///    "vetted": {
    ///      "description": "Whether or not the game counts towards the total completion percentage.\nThe encoding seems to be:\n* 1 means \"the game counts\"\n* 0 means \"the game does not count, but has achievements\"\n* null means \"the game does not count and has no achievements\"\n",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct AchievementProgress {
        ///The unique identifier for the game.
        pub appid: i64,
        ///The total number of achievements available in the game.
        pub total: i64,
        ///The number of achievements unlocked in the game.
        pub unlocked: i64,
        /**Whether or not the game counts towards the total completion percentage.
The encoding seems to be:
* 1 means "the game counts"
* 0 means "the game does not count, but has achievements"
* null means "the game does not count and has no achievements"
*/
        pub vetted: ::std::option::Option<i64>,
    }
    impl ::std::convert::From<&AchievementProgress> for AchievementProgress {
        fn from(value: &AchievementProgress) -> Self {
            value.clone()
        }
    }
    /**Steam's representation of a game.
The 'appid' field is the unique identifier for the game.
The 'name' field is the name of the game.
There are many more fields in the response,
but they are not relevant for the achievement progress.
The simplification allows to use the same schema for 'rgGames' and 'rgPerfectUnownedGames'.
*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Steam's representation of a game.\nThe 'appid' field is the unique identifier for the game.\nThe 'name' field is the name of the game.\nThere are many more fields in the response,\nbut they are not relevant for the achievement progress.\nThe simplification allows to use the same schema for 'rgGames' and 'rgPerfectUnownedGames'.\n",
    ///  "type": "object",
    ///  "properties": {
    ///    "appid": {
    ///      "description": "The unique identifier for the game.",
    ///      "type": "integer"
    ///    },
    ///    "name": {
    ///      "description": "The name of the game.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct Game {
        ///The unique identifier for the game.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub appid: ::std::option::Option<i64>,
        ///The name of the game.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&Game> for Game {
        fn from(value: &Game) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for Game {
        fn default() -> Self {
            Self {
                appid: Default::default(),
                name: Default::default(),
            }
        }
    }
    ///`GamesTable`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "achievement_progress",
    ///    "rgGames",
    ///    "rgPerfectUnownedGames"
    ///  ],
    ///  "properties": {
    ///    "achievement_progress": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AchievementProgress"
    ///      }
    ///    },
    ///    "rgGames": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Game"
    ///      }
    ///    },
    ///    "rgPerfectUnownedGames": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Game"
    ///      }
    ///    },
    ///    "strProfileName": {
    ///      "description": "The name of the Steam profile.",
    ///      "type": "string"
    ///    },
    ///    "strSteamId": {
    ///      "description": "The unique identifier for the Steam profile.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GamesTable {
        pub achievement_progress: ::std::vec::Vec<AchievementProgress>,
        #[serde(rename = "rgGames")]
        pub rg_games: ::std::vec::Vec<Game>,
        #[serde(rename = "rgPerfectUnownedGames")]
        pub rg_perfect_unowned_games: ::std::vec::Vec<Game>,
        ///The name of the Steam profile.
        #[serde(
            rename = "strProfileName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub str_profile_name: ::std::option::Option<::std::string::String>,
        ///The unique identifier for the Steam profile.
        #[serde(
            rename = "strSteamId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub str_steam_id: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&GamesTable> for GamesTable {
        fn from(value: &GamesTable) -> Self {
            value.clone()
        }
    }
}
