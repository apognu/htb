use super::{
    deserializers::string_or_struct,
    util::{HtbParser, HtbResponder},
};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub success: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub connection: Connection,
}

#[derive(Debug, Deserialize, Default)]
pub struct Connection {
    pub name: String,
    pub ip4: String,
    pub ip6: String,
}

pub async fn status() -> Result<Status, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url("/users/htb/connection/status"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}
