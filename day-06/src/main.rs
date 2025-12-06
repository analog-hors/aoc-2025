fn p1_parse_nums(line: &str) -> Vec<u64> {
    line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn p1_parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let (nums, ops) = input.trim().rsplit_once('\n').unwrap();
    let nums = nums.lines().map(p1_parse_nums).collect();
    let ops = ops.split_ascii_whitespace().collect();
    (nums, ops)
}

fn p2_parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let (nums, ops) = input.trim().rsplit_once('\n').unwrap();
    let ops = ops.split_ascii_whitespace().collect();

    let mut transposed_nums = Vec::new();
    for line in nums.lines() {
        transposed_nums.resize_with(line.len(), String::new);
        for (row, ch) in transposed_nums.iter_mut().zip(line.chars()) {
            row.push(ch);
        }
    }

    let mut grouped_nums = vec![Vec::new()];
    for row in transposed_nums {
        match row.trim().parse::<u64>() {
            Ok(n) => grouped_nums.last_mut().unwrap().push(n),
            Err(_) => grouped_nums.push(Vec::new()),
        }
    }

    (grouped_nums, ops)
}

fn part_1(input: String) -> u64 {
    let (nums, ops) = p1_parse_input(&input);
    (0..ops.len())
        .map(|j| {
            let column = (0..nums.len()).map(|i| nums[i][j]);
            match ops[j] {
                "+" => column.sum::<u64>(),
                "*" => column.product(),
                _ => panic!(),
            }
        })
        .sum()
}

fn part_2(input: String) -> u64 {
    let (nums, ops) = p2_parse_input(&input);
    nums.iter()
        .zip(ops)
        .map(|(nums, op)| {
            match op {
                "+" => nums.iter().sum::<u64>(),
                "*" => nums.iter().product(),
                _ => panic!(),
            }
        })
        .sum()
}

aoc::main!();
