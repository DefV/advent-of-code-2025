use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug, Eq, Hash, PartialEq)]
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
    let result = part1(&input, std::env::args().nth(2).unwrap().parse().unwrap());
    println!("Result: {}", result);
}

fn part1(input: &str, n: usize) -> usize {
    let points: Vec<Point> = input.lines().map(|line| line.into()).collect();

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

    let mut graph: HashMap<&Point, Vec<&Point>> = HashMap::new();
    for (_, p, q) in pairs[..n].into_iter() {
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

    sizes.iter().rev().take(3).product()
}
