use std::collections::HashMap;

fn main() {
    let input = aoc::input();

    println!("Step 1: {}", step1(&input));
}

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

fn step1(input: &str) -> usize {
    let mut paper_positions = Vec::new();
    let mut adjacency_count: HashMap<(usize, usize), usize> = HashMap::new();

    for (line, row) in input.lines().enumerate() {
        for (col, tile) in row.chars().enumerate() {
            let tile: Tile = tile.into();

            if matches!(tile, Tile::Paper) {
                paper_positions.push((col, line));

                ADJACENT_POSITIONS
                    .iter()
                    .filter_map(|(x, y)| {
                        Some((col.checked_add_signed(*x)?, line.checked_add_signed(*y)?))
                    })
                    .for_each(|(col, line)| {
                        *adjacency_count.entry((col, line)).or_insert(0) += 1;
                    });
            }
        }
    }

    paper_positions
        .into_iter()
        .filter(|(col, line)| *adjacency_count.get(&(*col, *line)).unwrap_or(&0) < 4)
        .count()
}

enum Tile {
    Paper,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            _ => panic!("Invalid tile character: {}", c),
        }
    }
}
