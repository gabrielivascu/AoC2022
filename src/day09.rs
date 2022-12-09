use std::collections::HashSet;

fn move_head(head: (i32, i32), dir: &str) -> (i32, i32) {
    let (mut x, mut y) = (head.0, head.1);
    match dir {
        "R" => x += 1,
        "L" => x -= 1,
        "U" => y -= 1,
        "D" => y += 1,
        _ => unreachable!(),
    }
    (x, y)
}

fn move_tail(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    let (xd, yd) = (lhs.0 - rhs.0, lhs.1 - rhs.1);
    // Check if the points are adjacent
    if xd.abs() > 1 || yd.abs() > 1 {
        (rhs.0 + xd.signum(), rhs.1 + yd.signum())
    } else {
        rhs
    }
}

fn solver(input: &str, rope_size: usize) -> usize {
    let mut rope = vec![(0, 0); rope_size]; // rope[0] is head
    let mut visited = HashSet::new();

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<usize>().unwrap();

        for _ in 0..steps {
            rope[0] = move_head(rope[0], dir);
            for k in 1..rope.len() {
                rope[k] = move_tail(rope[k - 1], rope[k]);
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

pub fn solve_1(input: &str) -> usize {
    solver(input, 2)
}

pub fn solve_2(input: &str) -> usize {
    solver(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day09-sample.txt")), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day09-sample.txt")), 1);
    }
}
