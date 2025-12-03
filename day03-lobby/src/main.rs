use std::str;

fn main() {
    let input = aoc::input();

    part1(&input);
}
fn part1(input: &str) {
    let result = input.lines()
                    .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>() )
                    .fold(0, |acc, line| acc + max_joltage(&line));

    println!("Result: {}", result);
}

fn max_joltage(line: &[u32]) -> u32 {
    let (max_index, decimal) = find_max_index(&line[..line.len() - 1]);
    let (_, unit) = find_max_index(&line[max_index + 1..]);

    decimal * 10 + *unit
}

fn find_max_index(line: &[u32]) -> (usize, &u32) {
    line.iter()
        .enumerate()
        .reduce(|(index, max), (i, num)| if num > max { (i, num) } else { (index, max) })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_joltage() {
        assert_eq!(
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        );
        assert_eq!(max_joltage(&[8, 1, 1, 1, 1, 1, 9]), 89);
    }
}
