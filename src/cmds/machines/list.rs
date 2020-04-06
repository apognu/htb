use crate::{api, args::MachineListArgs, cli};
use colored::*;
use prettytable::{format::consts::*, Cell, Row, Table};
use std::error::Error;

pub async fn list(args: &MachineListArgs) -> Result<(), Box<dyn Error>> {
    let mut machines = api::machines::list().await?;
    let spawned = api::machines::spawned().await?;
    let owned = api::machines::owns().await?;
    let todos = api::machines::todos().await?;
    let assigned = api::machines::assigned().await?;

    let mut table = Table::new();
    table.set_format(*FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.add_row(row!["", b -> "NAME", b -> "OS", b -> "USER", b -> "ROOT", b -> "IP ADDRESS", b -> "RATING", b -> "POINTS"]);

    machines.sort_by(|m1, m2| {
        let (f1, f2) = match args.sort.as_ref() {
            "name" => (m1.name.clone(), m2.name.clone()),
            "rating" => (m1.rating.clone(), m2.rating.clone()),
            "release" => (
                m1.release.clone().unwrap_or("Z".to_string()),
                m2.release.clone().unwrap_or("Z".to_string()),
            ),
            _ => (m1.id.to_string(), m2.id.to_string()),
        };

        if args.desc {
            f2.partial_cmp(&f1).unwrap()
        } else {
            f1.partial_cmp(&f2).unwrap()
        }
    });

    for machine in &machines {
        if let Some(difficulty) = &args.difficulty {
            let keep = match difficulty.as_ref() {
                "easy" => machine.points == 20,
                "medium" => machine.points == 30,
                "hard" => machine.points == 40,
                "insane" => machine.points == 50,
                _ => true,
            };

            if !keep {
                continue;
            }
        }
        if let Some(os) = &args.os {
            if machine.os.to_lowercase() != os.to_lowercase() {
                continue;
            }
        }
        if args.spawned && !spawned.contains(&machine.id) {
            continue;
        }
        if args.active && machine.retired {
            continue;
        }
        if args.retired && !machine.retired {
            continue;
        }
        if args.todo && !todos.contains(&machine.id) {
            continue;
        }
        if args.assigned && !assigned.contains(&machine.id) {
            continue;
        }
        if let Some(name) = &args.name {
            if !machine.name.to_lowercase().contains(&name.to_lowercase()) {
                continue;
            }
        }

        let user = owned.get(&machine.id).map(|(u, _)| u).unwrap_or(&false);
        let root = owned.get(&machine.id).map(|(_, r)| r).unwrap_or(&false);

        if args.owned && (!user || !root) {
            continue;
        }
        if args.unowned && *user && *root {
            continue;
        }

        let spawned = if spawned.contains(&machine.id) {
            Cell::new("⚫").style_spec("Fg")
        } else {
            Cell::new("⚫").style_spec("Fr")
        };

        table.add_row(Row::new(vec![
            spawned,
            Cell::new(&machine.name),
            Cell::new(&machine.os),
            Cell::new(&format!("👤 {}", machine.user_owns)).style_spec(if *user {
                "Fg"
            } else {
                ""
            }),
            Cell::new(&format!("＃ {}", machine.root_owns)).style_spec(if *root {
                "Fr"
            } else {
                ""
            }),
            Cell::new(&machine.ip),
            Cell::new(&format!("{} {}", "★".yellow(), machine.rating)),
            Cell::new(&format!("{} {}", "🞋".green(), machine.points)),
        ]));
    }

    if table.len() <= 1 {
        cli::error("No machine matches your criteria");
    } else {
        table.printstd();
    }

    Ok(())
}
