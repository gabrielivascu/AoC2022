use itertools::Itertools;

pub fn solve_1(input: &str) -> usize {
    solver(input, 4)
}

pub fn solve_2(input: &str) -> usize {
    solver(input, 14)
}

fn solver(input: &str, length: usize) -> usize {
    for i in length - 1..input.len() {
        if input[i + 1 - length..=i].chars().unique().count() == length {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day06-sample.txt")), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day06-sample.txt")), 19);
    }
}
