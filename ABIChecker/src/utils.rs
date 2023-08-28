use ansi_term::Color;

pub fn info_prompt(title: &str, msg: &str) {
    let title = format!("[{}]", title);
    println!(
        "{} {}",
        Color::Green.bold().paint(title),
        Color::White.paint(msg)
    );
}

pub fn warn_prompt(title: &str, msg: &str) {
    let title = format!("[{}]", title);
    println!(
        "{} {}",
        Color::Yellow.bold().paint(title),
        Color::White.paint(msg)
    );
}

pub fn error_prompt(title: &str, msg: &str) {
    let title = format!("[{}]", title);
    println!(
        "{} {}",
        Color::Red.bold().paint(title),
        Color::White.paint(msg)
    );
}
