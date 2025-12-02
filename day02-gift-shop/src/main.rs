use std::time::Instant;

fn main() {
    let input = aoc::input();

    let start = Instant::now();
    println!("Step 1:{}", step1(&input, is_double_number));
    println!("Time: {:?}", start.elapsed());

    let start = Instant::now();
    println!("Step 2:{}", step1(&input, has_recurring_digits));
    println!("Time: {:?}", start.elapsed());
}

fn step1(input: &str, checker: fn(u64) -> bool) -> u64 {
    input
        .split(',')
        .filter_map(|range_str| range_str.split_once('-'))
        .flat_map(|(start, stop)| {
            (start.parse::<u64>().unwrap()..=stop.parse::<u64>().unwrap()).filter(|&x| checker(x))
        })
        .sum::<u64>()
}

fn is_double_number(number: u64) -> bool {
    let num_digits = number.ilog10() + 1;
    let divider = 10_u64.pow(num_digits / 2);
    let (first, last) = (number / divider, number % divider);
    first == last
}

fn has_recurring_digits(number: u64) -> bool {
    let digits = number.to_string();
    let len = digits.len();

    (1..=len / 2).any(|chunk_size| {
        if len % chunk_size != 0 { return false; }

        let first = &digits[..chunk_size];
        (chunk_size..len).step_by(chunk_size).all(|i| &digits[i..i+chunk_size] == first)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_double_number() {
        assert!(is_double_number(1212));
        assert!(is_double_number(11));
        assert!(is_double_number(1188511885));

        assert!(!is_double_number(111));
    }

    #[test]
    fn test_has_recurring_digits() {
        assert!(has_recurring_digits(1212));
        assert!(has_recurring_digits(11));
        assert!(has_recurring_digits(111));
        assert!(has_recurring_digits(824824824));
        assert!(has_recurring_digits(1188511885));
    }
}
