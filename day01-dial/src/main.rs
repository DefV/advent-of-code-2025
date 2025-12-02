struct Dial {
    start: u64,
    size: u64,
}

impl Dial {
    fn rotate(&mut self, step: &str) -> u64 {
        let direction = &step[..1];
        let amount: u64 = step[1..].parse().unwrap();

        let mut rotations = amount / self.size;
        let remainder = amount % self.size;

        self.start = match direction {
            "L" => {
                match self.start.checked_sub(remainder) {
                    Some(new_start) => new_start,
                    None => {
                        // Edge case: if the start is 0, we don't need to add a rotation
                        if self.start != 0 { rotations += 1; }
                        self.start + self.size - remainder
                    }
                }
            },
            "R" => {
                let new_start = self.start + remainder;
                if new_start > self.size { rotations += 1 }
                new_start % self.size
            } 
            _ => panic!("Invalid direction: {}", direction),
        };

        rotations
    }
}

fn main() {
    let input = aoc::input();
    solve(&input);
}

fn solve(input: &str) {
    let mut dial = Dial {
        start: 50,
        size: 100,
    };

    let mut step1 = 0;
    let mut step2 = 0;
    input.lines().for_each(|line| {
        let rotations = dial.rotate(line);
        step2 += rotations;
        if dial.start == 0 {
            step1 += 1;
            step2 += 1;
        }
    });
    println!("Step 1: {}", step1);
    println!("Step 2: {}", step2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut dial = Dial {
            start: 50,
            size: 100,
        };

        dial.rotate("L68");
        assert_eq!(dial.start, 82);
        dial.rotate("L30");
        assert_eq!(dial.start, 52);
        dial.rotate("R48");
        assert_eq!(dial.start, 0);
    }
}
