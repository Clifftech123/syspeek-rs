use super::colors;

const LOGO: &str = r#"
 ___           ____            _
/ __|_  _ ___ |  _ \ ___  ___ | | __
\__ \ || (_-< | |_) / _ \/ _ \| |/ /
|___/\_, /__/ |  __/  __/  __/|   <
     |__/     |_|   \___|\___|_|\_\

"#;

pub fn banner(version: &str) {
    println!("{}", colors::title(LOGO));
    println!("  {} {}", colors::label("Version:"), colors::value(version));
    println!("  {} {}", colors::label("Created by:"), colors::value("Isaiah Clifford Opoku"));
    println!();
}

pub fn section_header(name: &str) {
    println!("  {}", colors::section(name));
}

pub fn key_value(key: &str, val: &str) {
    println!("    {:<20} {}", colors::label(key), colors::value(val));
}

pub fn progress_bar(label: &str, percent: f64) {
    let bar_width = 20;
    let filled = ((percent / 100.0) * bar_width as f64) as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "{}{}",
        colors::bar_filled(&"█".repeat(filled)),
        colors::bar_empty(&"░".repeat(empty)),
    );

    println!("    {:<20} {} {:.0}%", colors::label(label), bar, percent);
}

pub fn separator() {
    println!();
}
