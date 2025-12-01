fn parse_input(input: &str) -> impl Iterator<Item=i32> {
    input.lines().map(|line| {
        let (sign, magn) = line.trim().split_at(1);
        let sign = if sign == "L" { -1 } else { 1 };
        let magn = magn.parse::<i32>().unwrap();
        sign * magn
    })
}

fn part_1(input: String) -> usize {
    parse_input(&input)
        .scan(50, |dial, turn| {
            *dial = (*dial + turn).rem_euclid(100);
            Some(*dial)
        })
        .filter(|&dial| dial == 0)
        .count()
}

fn part_2(input: String) -> i32 {
    parse_input(&input)
        .scan(50, |dial, turn| {
            let (lo, hi) = match turn < 0 {
                true => (*dial + turn, *dial - 1),
                false => (*dial + 1, *dial + turn),
            };
            *dial = (*dial + turn).rem_euclid(100);
            Some(hi.div_euclid(100) - (lo - 1).div_euclid(100))
        })
        .sum()
}

aoc::main!();
