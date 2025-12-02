use std::time::Instant;
use chrono::Duration;
use colored::Colorize;

pub trait Day {
    pub fn run(&self) {
        let start = Instant::now();
        let p1 = self.part1();
        let mid = Instant::now();
        let p2 = self.part2();
        let end = Instant::now();

        println!("{}", header(&Self::day()))
    }

    fn part1(&self) -> isize { isize::MIN }
    fn part2(&self) -> isize { isize::MIN }

    fn day() -> String {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap()
            .replace("Day","")
    }
}

fn header(day: &str) -> String {
    format!("<===== Day {day} =====>")
        .green().to_string()
}
fn part(result: isize, time: Duration) -> String {

}
fn footer() -> String {
    format!("<=====================>")
        .green().to_string()
}
