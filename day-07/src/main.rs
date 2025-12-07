use std::collections::{HashSet, HashMap};

fn parse_input(input: &str) -> impl DoubleEndedIterator<Item=HashSet<i32>> {
    input.lines().map(|line| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .map(|(i, _)| i as i32)
            .collect()
    })
}

fn part_1(input: String) -> usize {
    let mut manifold = parse_input(&input);
    let mut beams = manifold.next().unwrap();

    let mut splits = 0;
    for splitters in manifold {
        for beam in std::mem::take(&mut beams) {
            if splitters.contains(&beam) {
                beams.insert(beam - 1);
                beams.insert(beam + 1);
                splits += 1;
            } else {
                beams.insert(beam);
            }
        }
    }

    splits
}

fn part_2(input: String) -> u64 {
    let manifold = parse_input(&input);

    let mut paths = HashMap::new();
    for (row, splitters) in manifold.rev().enumerate() {
        for col in splitters {
            let left = (0..row as i32).rev().find_map(|row| paths.get(&(row, col - 1)));
            let right = (0..row as i32).rev().find_map(|row| paths.get(&(row, col + 1)));
            paths.insert((row as i32, col), left.unwrap_or(&1) + right.unwrap_or(&1));
        }
    }

    *paths.iter().max().unwrap().1
}

aoc::main!();
