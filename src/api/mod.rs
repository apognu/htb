pub mod chat;
pub mod machines;
mod util;

use crate::cli;
use colored::*;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use std::{error::Error, fs};

const BASE_URL: &str = "https://www.hackthebox.eu/api";

pub fn client() -> Result<Client, Box<dyn Error>> {
    let token = match dirs::config_dir() {
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

    let creds = format!("Bearer {}", token);

    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&creds)?);

    Ok(ClientBuilder::new()
        .user_agent("htb/1.0.0 (https://github.com/apognu/htb")
        .default_headers(headers)
        .build()?)
}

pub fn url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}
