use crate::{api, cli};
use std::error::Error;

#[allow(dead_code)]
pub async fn new(usernames: &[String], message: &str) -> Result<(), Box<dyn Error>> {
    match api::chat::new(usernames, message).await {
        Ok(_) => cli::ok(&format!(
            "Conversation was created with {}",
            usernames.join(", ")
        )),

        Err(_) => cli::error("An error prevented us from creating this conversation"),
    }

    Ok(())
}

pub async fn send(id: u64, message: &str) -> Result<(), Box<dyn Error>> {
    let conversations = api::chat::list().await?;

    if !conversations.iter().any(|c| c.id == id) {
        cli::fatal("You do not have any conversation by that ID");
    }

    match api::chat::send(id, message).await {
        Ok(_) => cli::ok("Your message was sent successfully"),
        Err(message) => cli::error(&message.to_string()),
    }

    Ok(())
}
