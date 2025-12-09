use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{self, Debug};
#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',').map(|p| p.parse().unwrap());
        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Point {
    fn euclidean_distance(&self, other: &Point) -> f64 {
        let us = [self.x, self.y, self.z];
        let them = [other.x, other.y, other.z];

        (0..3)
            .fold(0.0, |acc, i| acc + (us[i] - them[i]).pow(2) as f64)
            .sqrt()
    }
}

fn main() {
    let input = aoc::input();
    let n = std::env::args().nth(2).unwrap().parse().unwrap();
    let result = part1(&input, n);
    println!("Result: {}", result);
    let result2 = part2(&input, n);
    println!("Result 2: {}", result2);
}

fn distance_pairs(points: &[Point]) -> Vec<(f64, &Point, &Point)> {
    let mut pairs: Vec<(f64, &Point, &Point)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            points
                .iter()
                .skip(i + 1)
                .map(move |q| (p.euclidean_distance(q), p, q))
        })
        .collect();

    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    pairs
}

fn size_of_networks(pairs: &[(f64, &Point, &Point)]) -> Vec<usize> {

    let mut graph: HashMap<&Point, Vec<&Point>> = HashMap::new();
    for (_, p, q) in pairs.iter() {
        graph.entry(p).or_default().push(q);
        graph.entry(q).or_default().push(p);
    }

    let mut visited: HashSet<&Point> = HashSet::new();
    let mut sizes = Vec::new();
    for node in graph.keys() {
        if visited.contains(node) {
            continue;
        }

        let mut queue: VecDeque<&Point> = VecDeque::new();
        queue.push_back(node);
        visited.insert(node);
        let mut size = 0;

        while let Some(p) = queue.pop_front() {
            size += 1;
            if let Some(neighbors) = graph.get(p) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        sizes.push(size);
    }

    sizes.sort_unstable();
    sizes
}
    
fn part1(input: &str, n: usize) -> usize {
    let points: Vec<Point> = input.lines().map(|line| line.into()).collect();

    let pairs = distance_pairs(&points);
    let sizes = size_of_networks(&pairs[..n]);
    sizes.iter().rev().take(3).product()
}

fn part2(input: &str, n: usize) -> usize {
    let points: Vec<Point> = input.lines().map(|line| line.into()).collect();

    let pairs = distance_pairs(&points);

    for i in n..pairs.len() {
        let sizes = size_of_networks(&pairs[..i]);
        if sizes.first() == Some(&points.len()) {
            let closing_pair = &pairs[i-1];
            
            return closing_pair.1.x * closing_pair.2.x
        }
    }

    unreachable!();
}
