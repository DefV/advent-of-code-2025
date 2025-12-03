fn main() {
    let input = aoc::input();

    println!("Part 1: {}", solve(&input, 2));
    println!("Part 2: {}", solve(&input, 12));
}

fn solve(input: &str, number_of_batteries: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, line| acc + max_joltage(&line, number_of_batteries))
}

fn max_joltage(line: &[u32], number_of_batteries: usize) -> u64 {
    let mut start = 0;
    let mut result = 0;
    for i in 1..=number_of_batteries {
        let (idx, &num) = find_max_index(&line[start..line.len() - number_of_batteries + i]);
        start += idx + 1;
        result = result * 10 + u64::from(num);
    }

    result
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
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            98
        );
        assert_eq!(max_joltage(&[8, 1, 1, 1, 1, 1, 9], 2), 89);
        assert_eq!(
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
    }
}
