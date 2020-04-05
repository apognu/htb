use colored::*;

pub fn ok(message: &str) {
    println!("{} {}", "✓".green(), message);
}

pub fn error(message: &str) {
    println!("{} {}", "✗".red(), message);
}

pub fn fatal(message: &str) -> ! {
    println!("{} {}", "✗".red(), message);

    std::process::exit(1)
}

pub fn holdon(message: &str) {
    println!("{} {}", "🕑".yellow(), message);
}
