use crate::{api, cli};
use std::error::Error;

pub async fn own(name: &str, flag: &str, difficulty: u8) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        let status = api::machines::own(machine.id, flag, difficulty * 10).await?;

        if status.success == 1 {
            cli::ok(&status.status);
        } else {
            cli::error(&status.status);
        }
    } else {
        cli::error("No machine was found by that name");
    }

    Ok(())
}
