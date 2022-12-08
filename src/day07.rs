use itertools::Itertools;
use std::collections::HashMap;

pub fn solve_1(input: &str) -> usize {
    compute_sizes(input)
        .values()
        .filter(|&x| *x <= 100_000)
        .sum()
}

pub fn solve_2(input: &str) -> usize {
    let sizes = compute_sizes(input);
    let free_space = 70_000_000 - sizes.get(&vec!["/"]).unwrap();
    // Sort the sizes in ascending order and find the first one that added to
    // the total free space amounts to more than 30000000.
    *sizes
        .values()
        .sorted()
        .find(|&x| free_space + x >= 30_000_000)
        .unwrap()
}

fn compute_sizes(input: &str) -> HashMap<Vec<&str>, usize> {
    // Current working directory as absolute path without '..'.
    let mut cwd = Vec::new();
    // Map absolute paths to sizes.
    let mut sizes = HashMap::new();

    for line in input.lines() {
        if line.starts_with("$ cd") {
            let dir = line.rsplit_once(' ').unwrap().1;
            if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir);
                sizes.insert(cwd.clone(), 0_usize);
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            // Not interesting.
        } else {
            let size = line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
            // Add the file size to each subpath in the absolute path.
            for k in 0..cwd.len() {
                *sizes.get_mut(&cwd[0..=k]).unwrap() += size;
            }
        }
    }

    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day07-sample.txt")), 95437);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day07-sample.txt")), 24933642);
    }
}
