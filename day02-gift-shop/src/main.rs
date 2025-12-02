fn main() {
    let input = aoc::input();

    println!("Step 1:{}", step1(&input, is_double_number));
    println!("Step 2:{}", step1(&input, has_recurring_digits));
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
    let num_digits = (number as f64).log10().floor() as u32 + 1;
    let divider = 10_u64.pow(num_digits / 2);
    let (first, last) = (number / divider, number % divider);
    first == last
}

fn has_recurring_digits(number: u64) -> bool {
    let n: Vec<char> = number.to_string().chars().collect();
    (1..=(n.len() / 2)).into_iter().any(|i| {
        let chunks: Vec<&[char]> = n.chunks(i).collect();
        chunks.windows(2).all(|w| w[0] == w[1])
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
