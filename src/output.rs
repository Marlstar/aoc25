use std::time::Duration;
use colored::Colorize;

pub fn header(day: &str) -> String {
    format!("<===== Day {day} =====>")
        .green().to_string()
}

pub fn part(result: isize, time: Duration) -> String {
    let time = format!("{:?}", time);
    format!(
        "Part 1: {} [{}]",
        result.to_string().blue(),
        time.red()
    )
}

pub fn footer(time: Duration) -> String {
    let timestr = format!("{:?}", time);
    let len = timestr.chars().count();
    let padding = 20 - len - 4;
    let pad_left = "=".repeat(padding/2);
    let pad_right = "=".repeat(padding/2 + padding % 2);
    format!("{}{} {} {}{}", "<".green(), pad_left.green(), timestr.red(), pad_right.green(), ">".green())
}
