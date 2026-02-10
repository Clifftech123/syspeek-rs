use colored::Colorize;

pub fn title(text: &str) -> String {
    text.bold().cyan().to_string()
}

pub fn label(text: &str) -> String {
    text.white().to_string()
}

pub fn value(text: &str) -> String {
    text.bold().green().to_string()
}

pub fn section(text: &str) -> String {
    text.bold().yellow().to_string()
}


pub fn bar_filled(text: &str) -> String {
    text.green().to_string()
}

pub fn bar_empty(text: &str) -> String {
    text.bright_black().to_string()
}
