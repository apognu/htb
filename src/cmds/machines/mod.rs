mod list;
mod own;
mod show;
mod state;
mod todo;

use crate::args::Machines::{self, *};
use std::error::Error;

pub async fn parse(subcommand: &Machines) -> Result<(), Box<dyn Error>> {
    match subcommand {
        List(args) => self::list::list(args).await,
        Show { name } => self::show::show(name).await,

        Own {
            name,
            flag,
            difficulty,
        } => self::own::own(name, flag, *difficulty).await,

        Todo { name } => self::todo::toggle(name).await,
        Reset { name } => self::state::reset(name).await,
        Start { name } => self::state::start(name).await,
        Stop { name } => self::state::stop(name).await,
    }
}
