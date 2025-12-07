fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let mut calculations: Vec<Vec<&str>> = input.lines().map(|s| s.split_whitespace().collect()).collect();
    let operations = calculations.pop().unwrap();
    let calculations: Vec<Vec<isize>> = calculations.into_iter().map(|c| c.into_iter().map(|s| s.parse::<isize>().unwrap()).collect()).collect();

    operations.into_iter().enumerate().map(|(i, operation)| {
        calculations.iter().map(|c| c[i] ).reduce(|acc, num| {
            from_op(operation).unwrap()(acc, num)
        }).unwrap()
    }).sum()
}

fn part2(input: &str) -> isize {
    let length = input.lines().next().unwrap().len();
    let mut total = 0;
    let mut numbers = vec![];
    let mut number = 0;

    for i in (0..length).rev() {
        for line in input.lines() {
            let char = line.chars().nth(i).unwrap();
            if let Some(digit) = char.to_digit(10) {
                number = number * 10 + digit as isize;
            } else if let Some(method) = from_op(&char.to_string()) {
                numbers.push(number);
                number = 0;
                total += numbers.into_iter().reduce(|acc, num| method(acc, num)).unwrap();
                numbers = vec![];
            }
        }
        if number > 0 {
            numbers.push(number);
        }
        number = 0;
    }
    
    total
}

fn from_op(op: &str) -> Option<fn(isize, isize) -> isize> {
    match op {
        "+" => Some(|a, b| a + b),
        "-" => Some(|a, b| a - b),
        "*" => Some(|a, b| a * b),
        "/" => Some(|a, b| a / b),
        _ => None
    }
}