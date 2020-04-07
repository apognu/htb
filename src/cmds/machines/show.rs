use crate::api::{self, HtbError};
use chrono::prelude::*;
use colored::*;
use std::error::Error;

pub async fn show(name: &str) -> Result<(), Box<dyn Error>> {
    let machine = api::machines::get_by_name(name).await?;

    if let Some(machine) = machine {
        let details = api::machines::get(machine.id).await?;
        let difficulty = api::machines::difficulty(machine.id).await?.round() as usize;

        println!("{}", machine.name.bold());

        let diffs = match machine.points {
            p if p <= 20 => ("Easy".green(), "■■".green()),
            p if p <= 30 => ("Medium".yellow(), "■■".yellow()),
            p if p <= 40 => ("Hard".red(), "■■".red()),
            p if p <= 50 => ("Insane".red(), "■■".red()),
            _ => ("N/A".dimmed(), "■■".normal()),
        };

        for _ in 0..difficulty {
            print!("{}", diffs.1);
        }

        for _ in difficulty..10 {
            print!("{}", "■■".black());
        }

        print!(" {} {}", "★".yellow(), machine.rating);

        println!();
        println!(
            "{} 💻 {} - {} {} - {} {} - {} {}",
            diffs.0,
            machine.os,
            "🞋".green(),
            machine.points,
            "👤".blue(),
            machine.user_owns,
            "＃".red(),
            machine.root_owns
        );
        println!();

        if let Some(maker2) = details.maker2 {
            println!(
                "Created by {} and {}",
                details.maker.name.green(),
                maker2.name.green()
            );
        } else {
            println!("Created by {}", details.maker.name.green());
        }

        if let Some(release) = machine.release {
            if let Ok(release) =
                NaiveDateTime::parse_from_str(&format!("{} 00:00:00", release), "%Y-%m-%d %H:%M:%S")
            {
                println!(
                    "Released on {}",
                    release.format("%d %b %Y").to_string().dimmed()
                );
            } else {
                println!("Released on {}", release.dimmed());
            }
        } else {
            println!("{}", "Unreleased".bold().red());
        }

        println!("IP address: {}", machine.ip.bold());
        println!();
        println!("{}", "First bloods:".bold());

        if let Some(user) = details.user_blood {
            println!(
                "  {} {} in {}",
                "👤".blue(),
                user.name.bold(),
                user.time.dimmed()
            );
        }

        if let Some(user) = details.root_blood {
            println!(
                "  {} {} in {}",
                "＃".red(),
                user.name.bold(),
                user.time.dimmed()
            );
        }

        Ok(())
    } else {
        Err(HtbError::boxed("No machine was found by that name"))
    }
}
