use crate::{
    api::{self, HtbError},
    cli,
};
use std::error::Error;

pub async fn own(name: &str, flag: &str, difficulty: u8) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        let status = api::machines::own(machine.id, flag, difficulty * 10).await?;

        if status.success == 1 {
            cli::ok(&status.status);

            Ok(())
        } else {
            Err(HtbError::boxed(status.status))
        }
    } else {
        Err(HtbError::boxed("No machine was found by that name"))
    }
}
