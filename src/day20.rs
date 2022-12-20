use std::cmp::Ordering;

pub fn solve_1(input: &str) -> i64 {
    solver(
        &input
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>(),
        1,
        1,
    )
}

pub fn solve_2(input: &str) -> i64 {
    solver(
        &input
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>(),
        10,
        811_589_153,
    )
}

fn solver(numbers: &[i64], iterations: usize, multiplier: i64) -> i64 {
    let numbers = numbers.iter().map(|x| x * multiplier).collect::<Vec<_>>();

    // Permutation of value indexes. Indexes are unique, so we don't have to
    // worry about duplicates in the input vector.
    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();

    for _ in 0..iterations {
        for (idx, x) in numbers.iter().enumerate() {
            let steps = match x.cmp(&0) {
                Ordering::Greater => usize::try_from(*x).unwrap() % (numbers.len() - 1),
                Ordering::Less => {
                    // Transform backward move into forward move.
                    (numbers.len() - 1) - (usize::try_from(-x).unwrap() % (numbers.len() - 1))
                }
                Ordering::Equal => 0,
            };
            move_forward(&mut indexes, idx, steps);
        }
    }

    let zero_idx = indexes
        .iter()
        .enumerate()
        .find(|(_, &x)| numbers[x] == 0)
        .unwrap()
        .0;

    numbers[indexes[(zero_idx + 1000) % numbers.len()]]
        + numbers[indexes[(zero_idx + 2000) % numbers.len()]]
        + numbers[indexes[(zero_idx + 3000) % numbers.len()]]
}

/// Nove @val forward by @count steps and wrap around the end of the vector.
fn move_forward(items: &mut [usize], val: usize, count: usize) {
    let mut idx = items.iter().enumerate().find(|(_, &x)| x == val).unwrap().0;
    let mut past_the_end = false;
    for _ in 0..count {
        if !past_the_end {
            past_the_end = idx == items.len() - 1;
        }
        let next = (idx + 1) % items.len();
        (items[next], items[idx]) = (items[idx], items[next]);
        idx = next;
    }
    if past_the_end {
        // Move one more time: [1, 2, 3, x] -> [x, 1, 2, 3]
        let end = items[items.len() - 1];
        for i in (1..items.len()).rev() {
            items[i] = items[i - 1];
        }
        items[0] = end;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day20-sample.txt")), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(include_str!("../input/day20-sample.txt")),
            1_623_178_306
        );
    }
}
