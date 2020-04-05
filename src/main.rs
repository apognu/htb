#![feature(async_closure)]
#![feature(box_syntax)]

#[macro_use]
extern crate prettytable;

mod api;
mod args;
mod cli;
mod cmds;

use args::Subcommand::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = args::parse();

    match options.subcommand {
        Config(ref sc) => cmds::config::parse(sc).await,
        Machines(ref sc) => cmds::machines::parse(sc).await,
        Chat(ref sc) => cmds::chat::parse(sc).await,
    }
}
