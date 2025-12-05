use std::ops::RangeInclusive;

fn main() {
    let input = aoc::input();

    let result = part1(&input);
    println!("Part 1: {}", result);
}

fn part1(input: &str) -> usize {
    let (range_body, ids_body) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = range_body
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap()))
        .map(|(start, end)| start..=end)
        .collect();

    let ids: Vec<usize> = ids_body
        .lines()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    ids.iter().filter(|id| ranges.iter().any(|range| range.contains(id))).count()
}
