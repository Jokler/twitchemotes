
/// This does not seem to be used right now
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Restrictions {
    pub channels: Vec<()>,
    pub games: Vec<()>,
}

/// Contains all the data we have about an `Emote`
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Emote<'a> {
    /// The ID used to identify the `Emote`
    pub id: &'a str,
    /// The name of the `Emote`
    pub code: &'a str,
    /// The channel of the creator
    pub channel: Option<&'a str>,
    /// If this fails to deserialize, tell me
    pub restrictions: Option<Restrictions>,
    /// The file extension of the `Emote`
    #[serde(rename="imageType")]
    pub image_type: &'a str,
}

/// Emotes which are available on every channel
pub mod global {
    use serde_json;
    use reqwest;

    use error::*;
    use super::Emote;

    /// A `HashMap` which lets you access an `Emote` via its name
    pub type Emotes<'a> = Vec<Emote<'a>>;

    /// The API response including a url template and the emotes
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct Global<'a> {
        /// The current status of the API
        pub status: i32,
        /// Template for all `Emote`s - {{id}} is the size e.g. `3x`
        #[serde(rename="urlTemplate")]
        pub url_template: String,
        /// A collection of all global emotes
        #[serde(borrow)]
        pub emotes: Emotes<'a>,
    }

    /// Downloads the API
    pub fn download_json() -> Result<reqwest::Response> {
        reqwest::get("https://api.betterttv.net/2/emotes").map_err(Error::from)
    }

    /// Deserialize json into `Global`
    pub fn from_str(json: &str) -> Result<Global> {
        let global: Global = serde_json::from_str(json).map_err(Error::from)?;

        Ok(global)
    }
}

/// Emotes which are enabled on a specific channel
pub mod channels {
    use super::Emote;
    use error::*;
    use reqwest;
    use serde_json;

    type Emotes<'a> = Vec<Emote<'a>>;

    /// The API Response including a url template and the emotes
    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct Channel<'a> {
        /// The current status of the API
        pub status: i32,
        /// Template for all `Emote`s - {{id}} is the size e.g. `3x`
        #[serde(rename="urlTemplate")]
        pub url_template: String,
        /// If this fails to deserialize, tell me
        pub bots: Vec<()>,
        /// A collection of the emotes on this channel
        #[serde(borrow)]
        pub emotes: Emotes<'a>,
    }

    /// Downloads the API
    pub fn download_json(name: &str) -> Result<reqwest::Response> {
        reqwest::get(&format!("https://api.betterttv.net/2/channels/{}", name)).map_err(Error::from)
    }

    /// Deserialize json into `Global`
    pub fn from_str(json: &str) -> Result<Channel> {
        let global = serde_json::from_str(json).map_err(Error::from)?;

        Ok(global)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::global;
    use super::channels;

    #[test]
    fn deserialize_channel() {
        let json = r#"{
            "status":200,
            "urlTemplate":"//cdn.betterttv.net/emote/{{id}}/{{image}}",
            "bots":[],
            "emotes":[
                {
                    "id":"594e207ae949fe3b435e5859",
                    "channel":"oshleyy",
                    "code":"pachiW",
                    "imageType":"png"
                },
                {
                    "id":"56e9f494fff3cc5c35e5287e",
                    "channel":"monkasen",
                    "code":"monkaS",
                    "imageType":"png"
                },
                {
                    "id":"55b6f480e66682f576dd94f5",
                    "channel":"turtlemaw",
                    "code":"Clap",
                    "imageType":"gif"
                },
                {
                    "id":"59c0b93f8798a53d5af9a484",
                    "channel":"oshleyy",
                    "code":"oshleyW",
                    "imageType":"png"
                },
                {
                    "id":"59c72b628798a53d5af9cbb4",
                    "channel":"oshleyy",
                    "code":"oshleyHmm",
                    "imageType":"png"
                }
            ]
        }"#;

        channels::from_str(json).unwrap();
    }

    #[test]
    #[ignore]
    fn deserialize_all_global_emotes() {
        let mut res = String::new();
        global::download_json().unwrap().read_to_string(&mut res).unwrap();
        global::from_str(&res).unwrap();
    }
}
