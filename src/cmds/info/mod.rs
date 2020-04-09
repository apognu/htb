mod status;

use crate::args::Info::{self, *};
use std::error::Error;

pub async fn parse(subcommand: &Info) -> Result<(), Box<dyn Error>> {
    match subcommand {
        Status => self::status::status().await,
    }
}
