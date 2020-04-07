use crate::{
    api::{self, HtbError},
    cli,
};
use colored::*;
use std::error::Error;

pub async fn toggle(name: &str) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        let todos = api::machines::toggle_todo(machine.id).await?;

        if todos.iter().any(|t| t.id == machine.id) {
            cli::ok(&format!(
                "`{}` was added to your todo list",
                machine.name.bold()
            ));
        } else {
            cli::ok(&format!(
                "`{}` was removed from your todo list",
                machine.name.bold()
            ));
        }

        Ok(())
    } else {
        Err(HtbError::boxed("No machine was found by that name"))
    }
}
