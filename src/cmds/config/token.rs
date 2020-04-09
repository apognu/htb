use crate::api::HtbError;
use std::{error::Error, fs};

pub async fn set_token(token: &str) -> Result<(), Box<dyn Error>> {
    match dirs::config_dir() {
        Some(config) => {
            let path = format!("{}/htb", config.display());

            fs::create_dir_all(&path)?;
            fs::write(format!("{}/token", &path), token)?;
        }

        None => {
            return Err(box HtbError::new(
                "Could not create directory under ~/.config/htb",
            ))
        }
    }

    Ok(())
}
