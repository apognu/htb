pub mod chat;
mod deserializers;
pub mod info;
pub mod machines;
mod util;

pub use self::util::HtbError;

use crate::cli;
use colored::*;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    redirect::Policy,
    Client, ClientBuilder,
};
use std::{error::Error, fs};

const BASE_URL: &str = "https://www.hackthebox.eu/api";

lazy_static! {
    static ref API_KEY: String = match dirs::config_dir() {
        Some(config) => {
            let path = format!("{}/htb/token", config.display());

            if let Ok(token) = fs::read_to_string(&path) {
                token
            } else {
                cli::fatal(&format!(
                    "could not read token, did you call `{}`?",
                    "htb config token".bold(),
                ))
            }
        }

        None => cli::fatal(&format!(
            "could not read token, did you call `{}`?",
            "htb config token".bold(),
        )),
    };
}

pub fn client() -> Result<Client, Box<dyn Error>> {
    let creds = format!("Bearer {}", *API_KEY);

    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&creds.trim())?);

    Ok(ClientBuilder::new()
        .user_agent("htb/1.0.0 (https://github.com/apognu/htb")
        .default_headers(headers)
        .redirect(Policy::none())
        .build()?)
}

pub fn url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}
