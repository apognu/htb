use colored::*;

pub fn info(message: &str) {
    println!("{} {}", "•".blue(), message);
}

pub fn ok(message: &str) {
    println!("{} {}", "✓".green(), message);
}

pub fn error(message: &str) {
    println!("{} {} {}", "✗".red(), "ERROR:".bold(), message);
}

pub fn fatal(message: &str) -> ! {
    println!("{} {} {}", "✗".red(), "ERROR:".bold(), message);

    std::process::exit(1)
}

pub fn holdon(message: &str) {
    println!("{} {}", "⧗".yellow(), message);
}
