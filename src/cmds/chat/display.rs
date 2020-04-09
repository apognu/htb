use super::util;
use crate::api;
use colored::*;
use std::error::Error;

pub async fn list() -> Result<(), Box<dyn Error>> {
    let conversations = api::chat::list().await?;

    for conversation in &conversations {
        println!(
            "💬 {} ({})",
            conversation.usernames.join(", ").bold(),
            conversation.id
        );
        println!(
            "   {}",
            util::decode_message(&conversation.lastmessage).dimmed()
        );
    }

    Ok(())
}

pub async fn show(id: u64) -> Result<(), Box<dyn Error>> {
    let messages = api::chat::get(id).await?;

    for message in &messages {
        println!(
            "{} {}: {}",
            message.time.dimmed(),
            message.username.bold(),
            util::decode_message(&message.text)
        );
    }

    Ok(())
}
