fn main() {
    let input = aoc::input();
    step1(&input);
}

fn step1(input: &str) {
    let result = input
        .split(',')
        .filter_map(|range_str| range_str.split_once('-'))
        .flat_map(|(start, stop)| {
            doubles_in_range(start.parse::<u64>().unwrap(), stop.parse::<u64>().unwrap())
        })
        .sum::<u64>();

    println!("Step 1:{}", result);
}

fn doubles_in_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end).filter(|&x| is_double_number(x)).collect()
}

fn is_double_number(number: u64) -> bool {
    let num_digits = (number as f64).log10().floor() as u32 + 1;
    let divider = 10_u64.pow(num_digits / 2);
    let (first, last) = (number / divider, number % divider);
    first == last
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
    fn test_doubles_in_range() {
        assert_eq!(doubles_in_range(11, 22), vec![11, 22]);
        assert_eq!(doubles_in_range(95, 115), vec![99]);
    }
}
