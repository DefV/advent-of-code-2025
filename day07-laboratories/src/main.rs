use std::collections::{HashSet, HashMap};

type Position = (usize, usize);

struct Map {
    start: Position,
    splitters: HashSet<Position>,
    height: usize,
}

fn parse_map(input: &str) -> Map {
    let mut start = Position::default();
    let mut splitters = HashSet::new();
    let height = input.lines().count();

    for (row, cols) in input.lines().enumerate() {
        for (col, chr) in cols.chars().enumerate() {
            match chr {
                'S' => start = (row, col),
                '^' => { splitters.insert((row, col)); },
                _ => (),
            }
        }
    }

    Map { start, splitters, height }
}

fn main() {
    let input = aoc::input();

    println!("Part 1 & 2: {:?}", part1(&input));
}

fn part1(input: &str) -> (usize, usize) {
    let map = parse_map(&input);
    let mut memory: HashMap<Position, usize> = HashMap::new();

    options(&map, map.start, &mut memory);

    (memory.len(), *memory.get(&(1, map.start.1)).unwrap())
}

fn options(map: &Map, position: Position, memory: &mut HashMap<Position, usize>) -> usize {
    let down = (position.0 + 1, position.1);
    if memory.contains_key(&down) {
        return *memory.get(&down).unwrap();
    }
    if map.splitters.contains(&down) {
        let result = options(map, (down.0, down.1 - 1), memory) + options(map, (down.0, down.1 + 1), memory);
        memory.insert(position, result);
        result
    }
    else if down.0 == map.height {
        1
    }
    else {
        options(&map, down, memory)
    }
}