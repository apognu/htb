#![feature(async_closure)]
#![feature(box_syntax)]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prettytable;

mod api;
mod args;
mod cli;
mod cmds;

use self::args::Subcommand::*;
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = args::parse();

    let result = match options.subcommand {
        Config(ref sc) => cmds::config::parse(sc).await,
        Machines(ref sc) => cmds::machines::parse(sc).await,
        Chat(ref sc) => cmds::chat::parse(sc).await,
    };

    if let Err(error) = result {
        cli::error(&error.to_string());
        process::exit(1);
    }

    Ok(())
}
