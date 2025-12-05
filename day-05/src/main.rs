fn parse_range(range: &str) -> (u64, u64) {
    let (lo, hi) = range.split_once('-').unwrap();
    (lo.parse().unwrap(), hi.parse().unwrap())
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges.lines().map(parse_range).collect();
    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();
    (ranges, ids)
}

fn part_1(input: String) -> usize {
    let (ranges, ids) = parse_input(&input);
    ids.iter().filter(|&id| ranges.iter().any(|(lo, hi)| lo <= id && id <= hi)).count()
}

fn part_2(input: String) -> u64 {
    let (mut ranges, _) = parse_input(&input);
    ranges.sort_unstable_by_key(|&(_, hi)| hi);

    let mut total = 0;
    while let Some((base_lo, base_hi)) = ranges.pop() {
        let mut next_lo = base_lo;
        ranges.retain(|&(lo, hi)| {
            if next_lo <= hi {
                next_lo = next_lo.min(lo);
                return false;
            }
            true
        });
        if next_lo < base_lo {
            ranges.push((next_lo, base_hi));
        } else {
            total += base_hi - base_lo + 1;
        }
    }

    total
}

aoc::main!();
