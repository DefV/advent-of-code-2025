fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> isize {
    let mut calculations: Vec<Vec<&str>> = input.lines().map(|s| s.split_whitespace().collect()).collect();
    let operations = calculations.pop().unwrap();
    let calculations: Vec<Vec<isize>> = calculations.into_iter().map(|c| c.into_iter().map(|s| s.parse::<isize>().unwrap()).collect()).collect();

    operations.into_iter().enumerate().map(|(i, operation)| {
        calculations.iter().map(|c| c[i] ).reduce(|acc, num| {
            from_op(operation)(acc, num)
        }).unwrap()
    }).sum()
}

fn from_op(op: &str) -> fn(isize, isize) -> isize {
    match op {
        "+" => |a, b| a + b,
        "-" => |a, b| a - b,
        "*" => |a, b| a * b,
        "/" => |a, b| a / b,
        _ => panic!("Invalid operation: {}", op),
    }
}