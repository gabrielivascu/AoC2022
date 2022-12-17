use std::collections::HashMap;

const CAVE_WIDTH: usize = 7;

pub fn solve_1(input: &str) -> usize {
    let rocks = get_rocks();
    let mut rocks = rocks.iter().cycle();
    let mut moves = input.trim().chars().cycle();
    let mut cave = Cave::new(10_000);

    for _ in 1..=2022 {
        let rock = rocks.next().unwrap();
        let (mut y_start, mut x_start) = (cave.height() + 3, 2);
        loop {
            let moved_x = match moves.next().unwrap() {
                '<' => x_start - 1,
                '>' => x_start + 1,
                _ => unreachable!(),
            };
            if cave.fits(rock, y_start, moved_x) {
                x_start = moved_x;
            }
            if y_start == 0 || !cave.fits(rock, y_start - 1, x_start) {
                break;
            }
            y_start -= 1;
        }
        cave.append_rock(rock, y_start, x_start);
    }

    cave.height()
}

pub fn solve_2(input: &str) -> usize {
    let rocks = get_rocks();
    let mut rocks = rocks.iter().enumerate().cycle();
    let mut moves = input.trim().chars().enumerate().cycle();
    let mut cave = Cave::new(10_000);

    let max_steps = 1_000_000_000_000_usize;
    let (mut step, mut total_height) = (0, 0);
    let mut cache = HashMap::new();

    while step < max_steps {
        let (rock_idx, rock) = rocks.next().unwrap();
        let (mut y_start, mut x_start) = (cave.height() + 3, 2);
        let move_idx;

        loop {
            let (idx, r#move) = moves.next().unwrap();
            let moved_x = match r#move {
                '<' => x_start - 1,
                '>' => x_start + 1,
                _ => unreachable!(),
            };
            if cave.fits(rock, y_start, moved_x) {
                x_start = moved_x;
            }
            if y_start == 0 || !cave.fits(rock, y_start - 1, x_start) {
                move_idx = idx;
                break;
            }
            y_start -= 1;
        }

        cave.append_rock(rock, y_start, x_start);

        let state = (rock_idx, move_idx, cave.get_column_heights());
        if let Some((idx, height)) = cache.get(&state) {
            let repeats = (max_steps - idx) / (step - idx) - 1;
            step += (step - idx) * repeats;
            total_height += (cave.height() - height) * repeats;
        } else {
            cache.insert(state, (step, cave.height()));
        }

        step += 1;
    }

    total_height + cave.height()
}

type Rock = Vec<(usize, usize)>;

fn get_rocks() -> [Rock; 5] {
    [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ]
}

#[derive(Debug)]
struct Cave {
    map: Vec<[char; CAVE_WIDTH]>,
}

impl Cave {
    fn new(capacity: usize) -> Cave {
        Cave {
            map: vec![['.'; CAVE_WIDTH]; capacity],
        }
    }

    fn height(&self) -> usize {
        self.map
            .iter()
            .position(|row| row == &['.'; CAVE_WIDTH])
            .unwrap()
    }

    fn fits(&self, rock: &Rock, y_start: usize, x_start: usize) -> bool {
        rock.iter()
            .all(|(y, x)| x_start + x < CAVE_WIDTH && self.map[y + y_start][x + x_start] != '#')
    }

    fn get_column_heights(&self) -> [usize; CAVE_WIDTH] {
        let mut heights = [0; CAVE_WIDTH];
        let height = self.height();
        for (x, value) in heights.iter_mut().enumerate() {
            *value = (0..height)
                .find(|&y| self.map[height - y][x] == '#')
                .unwrap_or(usize::MAX);
        }
        heights
    }

    fn append_rock(&mut self, rock: &Rock, y_start: usize, x_start: usize) {
        for (y, x) in rock {
            self.map[y_start + y][x_start + x] = '#';
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day17-sample.txt")), 3068);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(include_str!("../input/day17-sample.txt")),
            1514285714288
        );
    }
}
