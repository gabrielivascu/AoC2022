use std::collections::{HashMap, HashSet};

pub fn solve_1(input: &str) -> usize {
    solver(build_input_elves(input), false)
}

pub fn solve_2(input: &str) -> usize {
    solver(build_input_elves(input), true)
}

fn build_input_elves(input: &str) -> HashSet<Point> {
    let mut elves = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(Point::new(
                    i32::try_from(x).unwrap(),
                    i32::try_from(y).unwrap(),
                ));
            }
        }
    }
    elves
}

#[allow(clippy::too_many_lines)]
fn solver(mut elves: HashSet<Point>, part2: bool) -> usize {
    let mut facing = Direction::North;
    let num_elves = elves.len();

    for k in 1..=usize::MAX {
        let mut num_unmoved = 0;
        let mut proposals: HashMap<Point, Vec<Point>> = HashMap::new();

        for elf in &elves {
            if elf.neighbors().iter().all(|p| !elves.contains(p)) {
                num_unmoved += 1;
                continue;
            }

            for dir in facing.get_cycled() {
                match dir {
                    Direction::North => {
                        if [
                            Point::new(elf.x - 1, elf.y - 1),
                            Point::new(elf.x, elf.y - 1),
                            Point::new(elf.x + 1, elf.y - 1),
                        ]
                        .iter()
                        .all(|p| !elves.contains(p))
                        {
                            proposals
                                .entry(Point::new(elf.x, elf.y - 1))
                                .or_default()
                                .push(*elf);
                            break;
                        }
                    }
                    Direction::South => {
                        if [
                            Point::new(elf.x - 1, elf.y + 1),
                            Point::new(elf.x, elf.y + 1),
                            Point::new(elf.x + 1, elf.y + 1),
                        ]
                        .iter()
                        .all(|p| !elves.contains(p))
                        {
                            proposals
                                .entry(Point::new(elf.x, elf.y + 1))
                                .or_default()
                                .push(*elf);
                            break;
                        }
                    }
                    Direction::West => {
                        if [
                            Point::new(elf.x - 1, elf.y - 1),
                            Point::new(elf.x - 1, elf.y),
                            Point::new(elf.x - 1, elf.y + 1),
                        ]
                        .iter()
                        .all(|p| !elves.contains(p))
                        {
                            proposals
                                .entry(Point::new(elf.x - 1, elf.y))
                                .or_default()
                                .push(*elf);
                            break;
                        }
                    }
                    Direction::East => {
                        if [
                            Point::new(elf.x + 1, elf.y - 1),
                            Point::new(elf.x + 1, elf.y),
                            Point::new(elf.x + 1, elf.y + 1),
                        ]
                        .iter()
                        .all(|p| !elves.contains(p))
                        {
                            proposals
                                .entry(Point::new(elf.x + 1, elf.y))
                                .or_default()
                                .push(*elf);
                            break;
                        }
                    }
                }
            }
        }

        if part2 && num_unmoved == num_elves {
            return k;
        }

        for (new_pos, biders) in &proposals {
            if biders.len() == 1 {
                elves.remove(&biders[0]);
                elves.insert(*new_pos);
            }
        }

        facing = match facing {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        };

        if !part2 && k == 10 {
            let (mut x_min, mut y_min) = (i32::MAX, i32::MAX);
            let (mut x_max, mut y_max) = (i32::MIN, i32::MIN);
            for pos in elves {
                x_min = x_min.min(pos.x);
                y_min = y_min.min(pos.y);
                x_max = x_max.max(pos.x);
                y_max = y_max.max(pos.y);
            }
            return usize::try_from(x_max - x_min + 1).unwrap()
                * usize::try_from(y_max - y_min + 1).unwrap()
                - num_elves;
        }
    }

    0
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn neighbors(self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_cycled(self) -> [Direction; 4] {
        match self {
            Direction::North => [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            Direction::South => [
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
            ],
            Direction::West => [
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
            ],
            Direction::East => [
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day23-sample.txt")), 110);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day23-sample.txt")), 20);
    }
}
