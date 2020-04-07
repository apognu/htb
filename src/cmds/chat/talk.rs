use crate::{
    api::{self, HtbError},
    cli,
};
use std::error::Error;

#[allow(dead_code)]
pub async fn new(usernames: &[String], message: &str) -> Result<(), Box<dyn Error>> {
    match api::chat::new(usernames, message).await {
        Ok(_) => {
            cli::ok(&format!(
                "Conversation was created with {}",
                usernames.join(", ")
            ));

            Ok(())
        }

        Err(_) => Err(HtbError::boxed(
            "An error prevented us from creating this conversation",
        )),
    }
}

pub async fn send(id: u64, message: &str) -> Result<(), Box<dyn Error>> {
    let conversations = api::chat::list().await?;

    if !conversations.iter().any(|c| c.id == id) {
        return Err(HtbError::boxed(
            "You do not have any conversation by that ID",
        ));
    }

    api::chat::send(id, message).await?;

    cli::ok("Your message was sent successfully");

    Ok(())
}
