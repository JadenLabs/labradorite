use chrono;
use colored::Colorize;

fn raw_log(message: &str, level: String) {
    let iso_now = chrono::Local::now().to_rfc3339();
    println!(
        "{} [{}] {} {}",
        iso_now.dimmed(),
        level,
        ">>".blue(),
        message
    )
}

pub fn info(message: &str) {
    raw_log(message, "INFO".green().to_string());
}

// pub fn error(message: &str) {
//     raw_log(message, "ERRO".red().to_string());
// }