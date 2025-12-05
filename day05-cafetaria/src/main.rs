use std::ops::RangeInclusive;

fn main() {
    let input = aoc::input();

    let result = part1(&input);
    println!("Part 1: {}", result);

    let result2 = part2(&input);
    println!("Part 2: {}", result2);
}

fn part1(input: &str) -> usize {
    let (range_body, ids_body) = input.split_once("\n\n").unwrap();

    let ranges: Vec<RangeInclusive<usize>> = range_body
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| {
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .map(|(start, end)| start..=end)
        .collect();

    let ids: Vec<usize> = ids_body
        .lines()
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();

    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part2(input: &str) -> usize {
    let (range_body, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(usize, usize)> = range_body
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(start, end)| {
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect();

    ranges.sort_by_key(|(start, _)| *start);
    let merged_ranges = merge_overlapping_ranges(&ranges);
    
    merged_ranges.iter().map(|(start, end)| end - start + 1).sum()
}

// (1, 5), (2, 2)
fn merge_overlapping_ranges(ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut merged_ranges = vec![];

    for (i, (start, end)) in ranges.iter().enumerate() {
        let mut new_end = *end;

        if let Some((_, last_end)) = merged_ranges.last() && last_end >= start {
            continue;
        }

        for (start2, end2) in ranges.iter().skip(i + 1) {
            if start2 <= &new_end {
                new_end = new_end.max(*end2);
            }
        }
        merged_ranges.push((*start, new_end));
    }

    merged_ranges
}
