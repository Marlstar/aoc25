crate::line_day!(Day01);
impl crate::Day for Day01 {
    fn part1(&self) -> isize {
        let mut counter = 0;
        let mut pos = 50isize;
        for instruction in &self.input {
            match instruction.chars().next().unwrap() {
                'L' => pos -= instruction[1..].parse::<isize>().unwrap(),
                'R' => pos += instruction[1..].parse::<isize>().unwrap(),
                _ => panic!()
            }
            if pos % 100 == 0 { counter += 1 }
        }
        counter
    }
}
