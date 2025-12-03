use std::time::Instant;
use crate::output::*;

pub trait Day {
    fn run(&self) {
        let start = Instant::now();
        let p1 = self.part1();
        let mid = Instant::now();
        let p2 = self.part2();
        let end = Instant::now();

        println!();
        println!("{}", header(Self::day()));
        println!("{}", part(p1, mid-start));
        println!("{}", part(p2, end-mid));
        println!("{}", footer(end-start));
    }

    fn part1(&self) -> isize { isize::MIN }
    fn part2(&self) -> isize { isize::MIN }

    fn day() -> &'static str {
        let tn = std::any::type_name::<Self>();
        &tn[tn.len()-2..]
    }
}

#[macro_export]
macro_rules! day_impls {
    ($day:ident, $inputfn:ident) => {
        impl $day {
            pub fn new() -> Self {
                Self { input: $crate::input::$inputfn(Self::day()) }
            }
        }
        impl Default for $day {
            fn default() -> Self { Self::new() }
        }
    }
}

#[macro_export]
macro_rules! line_day {
    ($day:ident) => {
        use $crate::Day as _Day;
        pub struct $day {
            #[allow(dead_code)]
            input: Vec<String>,
        }
        $crate::day_impls!($day, lines);
    }
}

#[macro_export]
macro_rules! raw_day {
    ($day:ident) => {
        use $crate::Day as _Day;
        pub struct $day {
            #[allow(dead_code)]
            input: String,
        }
        $crate::day_impls($day, raw)
    }
}
