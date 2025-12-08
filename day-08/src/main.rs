use std::collections::{HashMap, HashSet};

type Node = [i64; 3];
type Graph = HashMap<Node, Vec<Node>>;

fn parse_input(input: &str) -> Graph {
    input.lines()
        .map(|line| {
            let mut parts = line.split(',').map(|n| n.parse().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            ([x, y, z], Vec::new())
        })
        .collect()
}

fn squared_distance([ax, ay, az]: Node, [bx, by, bz]: Node) -> i64 {
    (ax - bx).pow(2) + (ay - by).pow(2) + (az - bz).pow(2)
}

fn flood(graph: &Graph, visited: &mut HashSet<Node>, root: Node) {
    if visited.insert(root) {
        for &neighbour in graph.get(&root).unwrap() {
            flood(graph, visited, neighbour);
        }
    }
}

fn part_1(input: String) -> usize {
    let mut graph = parse_input(&input);

    let mut pairs = graph.keys()
        .flat_map(|&a| graph.keys().map(move |&b| (a, b)))
        .filter(|(a, b)| a < b)
        .collect::<Vec<_>>();

    pairs.sort_unstable_by_key(|&(a, b)| squared_distance(a, b));
    for &(a, b) in pairs.iter().take(1000) {
        graph.get_mut(&a).unwrap().push(b);
        graph.get_mut(&b).unwrap().push(a);
    }

    let mut circuit_sizes = Vec::new();
    let mut visited = HashSet::new();
    for &node in graph.keys() {
        let init_len = visited.len();
        flood(&graph, &mut visited, node);
        if visited.len() > init_len {
            circuit_sizes.push(visited.len() - init_len);
        }
    }

    circuit_sizes.sort_unstable();
    circuit_sizes.iter().rev().take(3).product()
}

fn part_2(input: String) -> i64 {
    let mut graph = parse_input(&input);

    let root = *graph.keys().next().unwrap();
    let mut root_component = HashSet::from([root]);

    let mut pairs = graph.keys()
        .flat_map(|&a| graph.keys().map(move |&b| (a, b)))
        .filter(|(a, b)| a < b)
        .collect::<Vec<_>>();

    pairs.sort_unstable_by_key(|&(a, b)| squared_distance(a, b));
    for (a, b) in pairs {
        graph.get_mut(&a).unwrap().push(b);
        graph.get_mut(&b).unwrap().push(a);
        if root_component.contains(&a) || root_component.contains(&b) {
            flood(&graph, &mut root_component, a);
            flood(&graph, &mut root_component, b);
        }
        if root_component.len() == graph.len() {
            return a[0] * b[0];
        }
    }

    panic!("empty graph")
}

aoc::main!();
