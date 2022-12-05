use itertools::Itertools;

pub fn solve_1(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum::<i64>())
        .max()
        .unwrap()
}

pub fn solve_2(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<i64>().unwrap()).sum::<i64>())
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day01-sample.txt")), 24000);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day01-sample.txt")), 45000);
    }
}
