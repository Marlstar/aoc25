use std::io::{prelude::*, BufReader};
use std::fs::File;

fn file(day: &str) -> File {
    File::open(format!("input/{day}.txt"))
        .expect("failed to read puzzle input file")
}

pub fn lines(day: &str) -> Vec<String> {
    BufReader::new(file(day))
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
}

pub fn raw(day: &str) -> String {
    let mut out = String::new();
    file(day).read_to_string(&mut out)
        .expect("failed to read input to string");
    out
}
