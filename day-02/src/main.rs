fn parse_input(input: &str) -> impl Iterator<Item=(u64, u64)> {
    input.split(',').map(|range| {
        let (lo, hi) = range.split_once('-').unwrap();
        (lo.parse().unwrap(), hi.parse().unwrap())
    })
}

fn repeat_digits(n: u64) -> u64 {
    let mut d = 10;
    while d <= n {
        d *= 10;
    }

    n * d + n
}

fn is_repeated(n: u64, k: u64) -> bool {
    if n % k < k / 10 {
        return false; 
    }

    let mut m = n;
    while m > 0 {
        if m % k != n % k {
            return false;
        }
        m /= k;
    }

    true
}

fn is_invalid(n: u64) -> bool {
    let mut k = 10;
    while k < n {
        if is_repeated(n, k) {
            return true;
        }
        k *= 10;
    }

    false
}

fn part_1(input: String) -> u64 {
    parse_input(&input)
        .flat_map(|(lo, hi)| {
            (1..)
                .map(repeat_digits)
                .skip_while(move |&n| n < lo)
                .take_while(move |&n| n <= hi)
        })
        .sum()
}

fn part_2(input: String) -> u64 {
    parse_input(&input)
        .flat_map(|(lo, hi)| (lo..=hi).filter(|&n| is_invalid(n)))
        .sum()
}

aoc::main!();
