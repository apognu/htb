use crate::api;
use colored::*;
use std::error::Error;

pub async fn status() -> Result<(), Box<dyn Error>> {
    let status = api::info::status().await?;

    if status.success == "1" {
        println!("VPN status  : {}", "CONNECTED".bold().green());
        println!("Username:     {}", status.connection.name);
        println!("IP addresses: {}", status.connection.ip4);
        println!("              {}", status.connection.ip6);
    } else {
        println!("VPN status: {}", "DISCONNECTED".bold().red());
    }

    Ok(())
}
