use crate::{api, cli};
use colored::*;
use std::error::Error;

pub async fn reset(name: &str) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        cli::holdon(&format!(
            "Please wait while we try and reset `{}`...",
            machine.name.bold(),
        ));

        let status = api::machines::reset(machine.id).await?;

        if status.success == 1 {
            cli::ok(&format!(" {}", status.status));
        } else {
            cli::error(&format!(" {}", status.status));
        }
    } else {
        cli::error("No machine was found by that name");
    }

    Ok(())
}

pub async fn start(name: &str) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        cli::holdon(&format!(
            "Please wait while we try and assign `{}` to you...",
            machine.name.bold(),
        ));

        let status = api::machines::start(machine.id).await?;

        if status.success == 1 {
            cli::ok(&format!(" {}", status.status));
        } else {
            cli::error(&format!(" {}", status.status));
        }
    } else {
        cli::error("No machine was found by that name");
    }

    Ok(())
}

pub async fn stop(name: &str) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        cli::holdon(&format!(
            "Please wait while we schedule `{}` for termination...",
            machine.name.bold()
        ));

        let status = api::machines::stop(machine.id).await?;

        if status.success == 1 {
            cli::ok(&format!(" {}", status.status));
        } else {
            cli::error(&format!(" {}", status.status));
        }
    } else {
        cli::error("No machine was found by that name");
    }

    Ok(())
}
