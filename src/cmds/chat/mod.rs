mod display;
mod shell;
mod talk;
mod util;

use crate::args::Chat::{self, *};
use std::error::Error;

pub async fn parse(subcommand: &Chat) -> Result<(), Box<dyn Error>> {
    match subcommand {
        List => self::display::list().await,
        Show { id } => self::display::show(*id).await,
        Send { id, message } => self::talk::send(*id, message).await,
        Open { id } => self::shell::open(*id).await,
    }
}
