use crate::{api, cli};
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
                "Makers: {} and {}",
                details.maker.name.dimmed(),
                maker2.name.dimmed()
            );
        } else {
            println!("Maker: {}", details.maker.name.dimmed());
        }

        println!("IP address: {}", machine.ip.dimmed());
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
    } else {
        cli::error("No machine was found by that name");
    }

    Ok(())
}
