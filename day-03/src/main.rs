fn parse_input(input: &str) -> impl Iterator<Item=Vec<u8>> {
    input.lines().map(|line| line.trim().bytes().map(|n| n - b'0').collect())
}

fn max_joltage(digits: &[u8], limit: usize) -> u64 {
    let mut max = vec![vec![0; limit + 1]; digits.len()];
    for i in 0..digits.len() {
        for j in 0..limit + 1 {
            if i == 0 && j == 0 {
                max[i][j] = 0;
            } else if i == 0 {
                max[i][j] = digits[i] as u64;
            } else if j == 0 {
                max[i][j] = max[i - 1][j];
            } else {
                max[i][j] = max[i - 1][j].max(max[i - 1][j - 1] * 10 + digits[i] as u64);
            }
        }
    }
    max[digits.len() - 1][limit]
}

fn part_1(input: String) -> u64 {
    parse_input(&input).map(|digits| max_joltage(&digits, 2)).sum()
}

fn part_2(input: String) -> u64 {
    parse_input(&input).map(|digits| max_joltage(&digits, 12)).sum()
}

aoc::main!();
