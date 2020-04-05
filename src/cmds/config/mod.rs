mod token;

use crate::args::Config::{self, *};
use std::error::Error;

pub async fn parse(subcommand: &Config) -> Result<(), Box<dyn Error>> {
    match subcommand {
        Token { token } => self::token::set_token(token).await,
    }
}
