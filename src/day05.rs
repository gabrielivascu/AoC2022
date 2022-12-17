pub fn solve_1(input: &str) -> String {
    let (mut stacks, max_length) = build_stacks(input);

    for line in input.lines().skip(max_length + 2) {
        let parts = line.split_once(" from ").unwrap();
        let count = parts.0.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let (src, dst) = parts.1.split_once(" to ").unwrap();
        let (src, dst) = (
            src.parse::<usize>().unwrap() - 1,
            dst.parse::<usize>().unwrap() - 1,
        );
        for _ in 0..count {
            let tmp = stacks[src].pop().unwrap();
            stacks[dst].push(tmp);
        }
    }

    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.last().unwrap());
    }
    output
}

pub fn solve_2(input: &str) -> String {
    let (mut stacks, max_length) = build_stacks(input);

    for line in input.lines().skip(max_length + 2) {
        let parts = line.split_once(" from ").unwrap();
        let count = parts.0.split_once(' ').unwrap().1.parse::<usize>().unwrap();
        let (src, dst) = parts.1.split_once(" to ").unwrap();
        let (src, dst) = (
            src.parse::<usize>().unwrap() - 1,
            dst.parse::<usize>().unwrap() - 1,
        );
        let mut tmp = Vec::new();
        for _ in 0..count {
            tmp.push(stacks[src].pop().unwrap());
        }
        for item in tmp.iter().rev() {
            stacks[dst].push(*item);
        }
    }

    let mut output = String::new();
    for stack in stacks {
        output.push(*stack.last().unwrap());
    }
    output
}

fn build_stacks(input: &str) -> (Vec<Vec<char>>, usize) {
    let mut max_length = 0;
    let mut num_stacks = 0;
    for line in input.lines() {
        if line.chars().nth(1).unwrap().is_ascii_digit() {
            num_stacks = line.split("   ").count();
            break;
        }
        max_length += 1;
    }
    assert!(
        num_stacks < 10,
        "input parsing for more than 9 stacks not supported"
    );

    let mut stacks = vec![Vec::new(); num_stacks];
    for line in input
        .lines()
        .take(max_length)
        .collect::<Vec<_>>()
        .iter()
        .rev()
    {
        let mut idx = 1;
        for stack in &mut stacks {
            let item = line.chars().nth(idx).unwrap();
            if item != ' ' {
                stack.push(item);
            }
            idx += 4;
        }
    }

    (stacks, max_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day05-sample.txt")), "CMZ");
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day05-sample.txt")), "MCD");
    }
}
