use super::util::{HtbError, HtbParser, HtbResponder};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Conversation {
    pub id: u64,
    pub usernames: Vec<String>,
    pub lastmessage: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: u64,
    pub username: String,
    pub rank: String,
    pub text: String,
    pub time: String,
}

pub async fn list() -> Result<Vec<Conversation>, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url("/conversations/list"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn get(id: u64) -> Result<Vec<Message>, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/conversations/load/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

#[derive(Debug, Serialize)]
struct NewConversation {
    recipients: Vec<String>,
    message: String,
}

#[allow(dead_code)]
pub async fn new(recipients: &[String], message: &str) -> Result<(), Box<dyn Error>> {
    let body = NewConversation {
        recipients: recipients.to_owned(),
        message: message.to_owned(),
    };

    let api = super::client()?;
    api.post(&super::url("/conversations/new"))
        .json(&body)
        .send()
        .await
        .check()?;

    Ok(())
}

pub async fn send(id: u64, message: &str) -> Result<(), Box<dyn Error>> {
    let id = id.to_string();
    let params = [("id", id.as_ref()), ("message", message)];

    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/conversations/send/{}", id)))
        .form(&params)
        .send()
        .await
        .check()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(box HtbError::new(response.json::<String>().await?))
    }
}
