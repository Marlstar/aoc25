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

    fn part2(&self) -> isize {
        let mut counter = 0;
        let mut pos = 50isize;
        for instruction in &self.input {
            let ipos = pos;
            let coeff = match instruction.chars().next().unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!()
            };
            let mag = instruction[1..].parse::<isize>().unwrap();
            let delta100 = if coeff > 0 { 100 - ipos }
            else if ipos == 0 { 100 }
            else { ipos };
            let offset = mag - delta100;
            let val = if offset >= 0 { 1 + offset/100 } else { 0 };
            counter += val;

            pos = (pos + mag * coeff) % 100;
            if pos < 0 { pos += 100 }

            if coeff > 0 {
                eprintln!("{ipos} + {mag} => {pos} (+{val})")
            } else { eprintln!("{ipos} - {mag} => {pos} (+{val})")}
        }
        counter
    }
}
