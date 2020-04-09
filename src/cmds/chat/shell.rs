use super::util;
use crate::{
    api::{self, chat::Conversation},
    cli,
};
use colored::*;
use libnotify::Notification;
use linefeed::{DefaultTerminal, Interface, ReadResult};
use std::{
    error::Error,
    io::stdout,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};
use tokio::time::delay_for;

pub async fn open(id: u64) -> Result<(), Box<dyn Error>> {
    cli::ok("Chat session started");
    println!("  {} to display latest messages", "/history".bold());
    println!("  {} or {} to exit", "/quit".bold(), "^D".bold());

    let conversation = Arc::new(
        api::chat::list()
            .await?
            .into_iter()
            .find(|c| c.id == id)
            .unwrap(),
    );

    let messages = api::chat::get(id).await?;

    let last_seen_id = Arc::new(AtomicU64::new(
        messages.iter().map(|m| m.id).max().unwrap_or(0),
    ));

    for message in &*messages {
        println!(
            "{} {}: {}",
            message.time.dimmed(),
            message.username.bold(),
            util::decode_message(&message.text)
        );
    }

    let rl = Arc::new(Interface::new("")?);
    rl.set_prompt("> ")?;

    {
        let conversation = Arc::clone(&conversation);
        let last_seen_id = Arc::clone(&last_seen_id);
        let w = Arc::clone(&rl);

        tokio::spawn(async move {
            loop {
                delay_for(Duration::from_secs(10)).await;
                print_all_messages(&conversation, &last_seen_id, &w, false).await;
            }
        });
    }

    let _ = libnotify::init("Hack The Box");

    loop {
        match rl.read_line()? {
            ReadResult::Input(line) => {
                let (_, x) = {
                    let mut stdout = stdout().into_raw_mode().unwrap();
                    stdout.cursor_pos().unwrap()
                };

                let message = line.trim();

                if message.is_empty() {
                    continue;
                }

                println!(
                    "{}{}{}",
                    termion::cursor::Goto(1, x - 1),
                    termion::clear::UntilNewline,
                    termion::cursor::Goto(1, x - 2)
                );

                match message {
                    "/quit" => break,

                    "/history" => {
                        cli::ok("Printing your history below");

                        print_all_messages(&conversation, &last_seen_id, &rl, true).await;
                    }

                    _ => {
                        let conversation = Arc::clone(&conversation);
                        let last_seen_id = Arc::clone(&last_seen_id);
                        let w = Arc::clone(&rl);

                        tokio::spawn(async move {
                            api::chat::send(id, &line).await.unwrap();
                            print_all_messages(&conversation, &last_seen_id, &w, false).await;
                        });
                    }
                }
            }

            ReadResult::Eof => break,

            _ => (),
        }
    }

    Ok(())
}

async fn print_all_messages(
    conversation: &Arc<Conversation>,
    last_seen_id: &Arc<AtomicU64>,
    w: &Arc<Interface<DefaultTerminal>>,
    all: bool,
) {
    let messages = api::chat::get(conversation.id).await.unwrap();
    let mut w = w.lock_writer_erase().unwrap();

    for message in &messages {
        let seen = message.id <= last_seen_id.load(Ordering::Relaxed);

        if all || !seen {
            let body = util::decode_message(&message.text);
            writeln!(
                w,
                "{} {}: {}",
                message.time.dimmed(),
                message.username.bold(),
                body
            )
            .unwrap();

            if conversation.usernames.contains(&message.username) && !seen {
                let notification = Notification::new(
                    &format!("Hack The Box: new message from {}", message.username),
                    Some(body.as_ref()),
                    None,
                );

                notification.show().unwrap();
            }

            if !all || message.id > last_seen_id.load(Ordering::Relaxed) {
                last_seen_id.store(message.id, Ordering::Relaxed);
            }
        }
    }
}
