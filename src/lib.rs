//! Functions/Structures to interact with twitchemotes and BTTV emotes

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![deny(unsafe_code)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

extern crate serde;
extern crate serde_json;

extern crate reqwest;

/// Errors for the twitchemotes crate using `error_chain`
pub mod error {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::error::Error);
            Reqwest(::reqwest::Error);
        }
    }
}

/// Interface for the twitchemotes.com API
pub mod ttv;
/// Interface for the unofficial BTTV API
pub mod bttv;
