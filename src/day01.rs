use itertools::Itertools;

pub fn solve_1() -> i64 {
    include_str!("../input/day01.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum::<i64>())
        .max()
        .unwrap()
}

pub fn solve_2() -> i64 {
    include_str!("../input/day01.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum::<i64>())
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum()
}
