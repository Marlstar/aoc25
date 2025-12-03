use aoc25::Day;

fn main() {
    let day = std::env::args().nth(1).expect("provide a day")
        .parse::<usize>().unwrap();

    match day {
        1 => aoc25::days::Day01::new().run(),
        2 => aoc25::days::Day02::new().run(),
        3 => aoc25::days::Day03::new().run(),
        4 => aoc25::days::Day04::new().run(),
        5 => aoc25::days::Day05::new().run(),
        6 => aoc25::days::Day06::new().run(),
        7 => aoc25::days::Day07::new().run(),
        8 => aoc25::days::Day08::new().run(),
        9 => aoc25::days::Day09::new().run(),
        10 => aoc25::days::Day10::new().run(),
        11 => aoc25::days::Day11::new().run(),
        12 => aoc25::days::Day12::new().run(),
        _ => { panic!("invalid day"); }
    };
}
