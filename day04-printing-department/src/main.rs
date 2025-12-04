use std::collections::HashSet;

const ADJACENT_POSITIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const PAPER_CHAR: char = '@';

type Position = (usize, usize);
struct Warehouse {
    paper_positions: HashSet<Position>,
}

impl Warehouse {
    fn from_input(input: &str) -> Self {
        let paper_positions = input
            .lines()
            .enumerate()
            .flat_map(|(line, row)| {
                row.chars().enumerate().filter_map(move |(col, tile)| {
                    if tile == PAPER_CHAR {
                        Some((col, line))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { paper_positions }
    }

    fn reachable_positions(&self) -> HashSet<Position> {
        self.paper_positions.clone().iter().filter_map(|(col, line)| {
            let adjacent_positions = ADJACENT_POSITIONS.iter().filter_map(|(x, y)| {
                Some((col.checked_add_signed(*x)?, line.checked_add_signed(*y)?))
            }).collect::<HashSet<Position>>();

            if self.paper_positions.intersection(&adjacent_positions).count() < 4 {
                Some((*col, *line))
            } else {
                None
            }
        }).collect()
    }

    fn remove_reachable_positions(&self, reachable_positions: HashSet<Position>) -> Self {
        Self {
            paper_positions: self.paper_positions.difference(&reachable_positions).cloned().collect(),
        }
    }
}

fn main() {
    let input = aoc::input();
    let mut warehouse = Warehouse::from_input(&input);

    println!("Step 1: {}", warehouse.reachable_positions().len());

    let mut removed_positions = 0;

    while let reachable_positions = warehouse.reachable_positions() && reachable_positions.len() > 0 {
        removed_positions += reachable_positions.len();
        warehouse = warehouse.remove_reachable_positions(reachable_positions);
    }

    println!("Step 2: {}", removed_positions);
}