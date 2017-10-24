//!
//! The API does not contain a template URL to download emotes from
//! so this crate does not provide it either.
//! If you need to download emotes you can try using
//! <https://static-cdn.jtvnw.net/emoticons/v1/{id}/{size}>
//! as your template.
//! To use it replace {id} with the id of the [`Emote`]
//! and {size} with 1.0, 2.0 or 3.0 for the respective resolution.
//!
//! Keep in mind that many emotes were made for the 1.0 size.
//! [`Emote`]: global/struct.Emote.html

use std::io::Read;
use reqwest;

use error::*;

fn download(url: &str) -> Result<String> {
    let mut resp = reqwest::get(url).map_err(Error::from)?;

    let mut content = String::new();
    resp.read_to_string(&mut content)
        .map_err(Error::from)?;
    Ok(content)
}

/// Contains all the data we have about an `Emote`
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Emote<'a> {
    /// The ID used to identify the emote
    pub id: i32,
    /// The name of the `Emote`
    pub code: &'a str,
    /// The set which this emote belongs to
    pub emoticon_set: i32,
    /// Describes the emote - for now it seems to always be None
    pub description: Option<&'a str>,
}

/// Global Emote API
pub mod global {
    use std::collections::HashMap;
    use serde_json;

    use error::*;
    use super::Emote;
    use super::download;

    /// A `HashMap` which lets you access an `Emote` via its name
    pub type Emotes<'a> = HashMap<&'a str, Emote<'a>>;

    /// Downloads the API
    pub fn download_json() -> Result<String> {
        download("https://twitchemotes.com/api_cache/v3/global.json")
    }
    /// Deserialize json into `Emotes`
    pub fn from_str(json: &str) -> Result<Emotes> {
        let global: Emotes = serde_json::from_str(json).map_err(Error::from)?;

        Ok(global)
    }
}

/// Subscriber Emote API
pub mod subscriber {
    use std::collections::HashMap;

    use serde_json;
    use serde::{Deserializer, Deserialize};

    use error::*;
    use super::Emote;
    use super::download;

    /// Information about either a subscriber or a bits badge
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct Badge<'a> {
        /// 1x Resolution
        pub image_url_1x: String,
        /// 2x Resolution
        pub image_url_2x: String,
        /// 4x Resolution
        pub image_url_4x: String,
        /// Describes the badge
        pub description: &'a str,
        /// The title of the badge
        pub title: &'a str,
        pub click_action: &'a str,
        #[serde(deserialize_with="optional_string")]
        pub click_url: Option<String>,
    }

    fn optional_string<'de, D>(deserializer: D) -> ::std::result::Result<Option<String>, D::Error>
        where D: Deserializer<'de>
    {
        String::deserialize(deserializer).map(|o| if o.is_empty() { None } else { Some(o) })
    }

    /// Represents a collection of `Plan`s
    pub type Plans<'a> = HashMap<&'a str, Option<&'a str>>;
    /// Represents a collection of `Emote`s
    pub type Emotes<'a> = Vec<Emote<'a>>;
    /// Represents a collection of `Badge`s
    pub type Badges<'a> = HashMap<&'a str, Badge<'a>>;

    /// Contains available resolutions for a Cheermote
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct Cheermote {
        /// 1x Resolution
        #[serde(rename="1")]
        pub url_1: String,
        /// 1.5x Resolution
        #[serde(rename="1.5")]
        pub url_1_5: String,
        /// 2x Resolution
        #[serde(rename="2")]
        pub url_2: String,
        /// 3x Resolution
        #[serde(rename="3")]
        pub url_3: String,
        /// 4x Resolution
        #[serde(rename="4")]
        pub url_4: String,
    }

    /// Represents a collection of `Cheermote`s
    pub type Cheermotes<'a> = HashMap<&'a str, Cheermote>;

    /// Contains everything we know about a channel
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct Channel<'a> {
        /// The name of the channel
        pub channel_name: &'a str,
        /// The name that is displayed to users
        pub display_name: &'a str,
        /// The Channel ID
        pub channel_id: &'a str,
        /// The type of channel e.g. partner
        pub broadcaster_type: Option<&'a str>,
        /// A `HashMap` which contains Prices and optional IDs as Keys/Values respectively
        pub plans: Plans<'a>,
        /// A collection of the emotes which belong to this channel
        pub emotes: Emotes<'a>,
        /// A collection of the subscriber_badges which belong to this channel
        pub subscriber_badges: Option<Badges<'a>>,
        /// A collection of the bits_badges which belong to this channel
        pub bits_badges: Option<Badges<'a>>,
        /// A collection of the Cheermotes which belong to this channel
        pub cheermotes: Option<Cheermotes<'a>>,
        pub base_set_id: &'a str,
    }

    /// A `HashMap` which lets you access a `Channel` via its ID
    pub type Channels<'a> = HashMap<&'a str, Channel<'a>>;

    /// Downloads the API
    pub fn download_json() -> Result<String> {
        download("https://twitchemotes.com/api_cache/v3/subscriber.json")
    }
    /// Deserialize json into `Channels`
    pub fn from_str(json: &str) -> Result<Channels> {
        let channels: Channels = serde_json::from_str(json).map_err(Error::from)?;

        Ok(channels)
    }
}

#[cfg(test)]
mod tests {
    use super::Emote;
    use super::global;
    use super::subscriber;

    #[test]
    fn deserialize_emote() {
        let json = r#"{
	          "emote1": {
		            "id": 91735,
		            "code": "emote1",
		            "emoticon_set": 0,
		            "description": null
	          }
        }"#;
        let emotes = global::from_str(json).unwrap();

        assert_eq!(emotes.get("emote1").unwrap(),
                   &Emote {
                        id: 91735,
                        code: "emote1",
                        emoticon_set: 0,
                        description: None,
                    });
    }

    #[test]
    #[ignore]
    fn deserialize_all_global_emotes() {
        let res = global::download_json().unwrap();
        global::from_str(&res).unwrap();
    }

    #[test]
    #[ignore]
    fn deserialize_all_subscriber_emotes() {
        let res = subscriber::download_json().unwrap();
        subscriber::from_str(&res).unwrap();
    }
}
