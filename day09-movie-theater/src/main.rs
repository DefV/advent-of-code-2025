use anyhow::{Result, anyhow};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (x, y) = s.split_once(",").ok_or(anyhow!("Invalid point: {}", s))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl Point {
    fn area(&self, point: &Point) -> usize {
        (self.x.abs_diff(point.x) + 1) * (self.y.abs_diff(point.y) + 1)
    }
}

fn main() {
    let input = aoc::input();
    let result = part1(&input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> usize {
    let points: Vec<Point> = input
        .lines()
        .map(|line| Point::try_from(line).unwrap())
        .collect();

    points.iter().enumerate().flat_map(|(i, point)| {
        points.iter().skip(i + 1).map(|p| point.area(p))
    }).max().unwrap()
}
