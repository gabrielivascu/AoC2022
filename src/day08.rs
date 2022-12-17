use itertools::iproduct;

pub fn solve_1(input: &str) -> usize {
    let (grid, height, width) = build_grid(input);

    let mut count = width * 2 + height * 2 - 4;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if (0..y).all(|t| grid[t][x] < grid[y][x])
                || (y + 1..height).all(|t| grid[t][x] < grid[y][x])
                || (0..x).all(|t| grid[y][t] < grid[y][x])
                || (x + 1..width).all(|t| grid[y][t] < grid[y][x])
            {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_2(input: &str) -> i32 {
    let (grid, height, width) = build_grid(input);

    iproduct!(1..height - 1, 1..width - 1)
        .map(|(y, x)| {
            let (mut score_up, mut idx) = (0, y - 1);
            loop {
                score_up += 1;
                if idx == 0 || grid[idx][x] >= grid[y][x] {
                    break;
                }
                idx -= 1;
            }
            let (mut score_down, mut idx) = (0, y + 1);
            loop {
                score_down += 1;
                if idx == height - 1 || grid[idx][x] >= grid[y][x] {
                    break;
                }
                idx += 1;
            }
            let (mut score_left, mut idx) = (0, x - 1);
            loop {
                score_left += 1;
                if idx == 0 || grid[y][idx] >= grid[y][x] {
                    break;
                }
                idx -= 1;
            }
            let (mut score_right, mut idx) = (0, x + 1);
            loop {
                score_right += 1;
                if idx == width - 1 || grid[y][idx] >= grid[y][x] {
                    break;
                }
                idx += 1;
            }
            score_up * score_down * score_left * score_right
        })
        .max()
        .unwrap()
}

fn build_grid(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let mut grid = vec![];
    let (width, mut height) = (input.lines().next().unwrap().len(), 0_usize);
    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
        height += 1;
    }
    (grid, height, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day08-sample.txt")), 21);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day08-sample.txt")), 8);
    }
}
