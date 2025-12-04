use std::collections::{HashSet, BTreeSet};

type Cell = (i32, i32);
type Grid = HashSet<Cell>;

fn parse_input(input: &str) -> Grid {
    input.lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, cell)| cell == '@')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect()
}

fn neighbours((x, y): Cell) -> [Cell; 8] {
    [
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1)
    ]
}

fn accessible(grid: &Grid, cell: Cell) -> bool {
    neighbours(cell).iter().filter(|neigh| grid.contains(neigh)).count() < 4
}

fn part_1(input: String) -> usize {
    let grid = parse_input(&input);
    grid.iter().filter(|&&cell| accessible(&grid, cell)).count()
}

fn part_2(input: String) -> usize {
    let mut grid = parse_input(&input);
    let init_rolls = grid.len();

    let mut to_check = BTreeSet::from_iter(grid.iter().copied());
    while let Some(cell) = to_check.pop_first() {
        if accessible(&grid, cell) && grid.remove(&cell) {
            to_check.extend(neighbours(cell));
        }
    }

    init_rolls - grid.len()
}

aoc::main!();
