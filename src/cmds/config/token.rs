use crate::cli;
use std::{error::Error, fs};

pub async fn set_token(token: &str) -> Result<(), Box<dyn Error>> {
    match dirs::config_dir() {
        Some(config) => {
            let path = format!("{}/htb", config.display());
            fs::create_dir_all(&path)?;
            fs::write(format!("{}/token", &path), token)?;
        }

        None => cli::error("Could not create directory under ~/.config/htb"),
    }

    Ok(())
}
