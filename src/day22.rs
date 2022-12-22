use std::collections::HashMap;

pub fn solve_1(input: &str) -> usize {
    let (map, mut pos, instructions) = build_input(input);

    let mut facing = Facing::Right;
    for instr in &instructions {
        match instr {
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    match get_next_pos(&map, &pos, facing) {
                        Some(new_pos) => pos = new_pos,
                        None => {
                            break;
                        }
                    }
                }
            }
            Instruction::Rotate(r) => {
                facing = facing.rotate(*r);
            }
        }
    }

    pos.y * 1000 + pos.x * 4 + facing.to_score()
}

pub fn solve_2(_input: &str) -> usize {
    0
}

fn build_input(input: &str) -> (HashMap<Point, char>, Point, Vec<Instruction>) {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            if c != ' ' {
                map.insert(Point::new(x + 1, y + 1), c);
            }
        }
    }

    let mut start = Point::new(0, 0);
    for x in get_min_x(&map, 1)..=usize::MAX {
        if *map.get(&Point::new(x, 1)).unwrap() == '.' {
            start = Point::new(x, 1);
            break;
        }
    }

    let mut num = 0_usize;
    let mut instructions = vec![];
    for c in input.lines().rev().next().unwrap().chars() {
        match c.to_digit(10) {
            Some(x) => {
                num = num * 10 + usize::try_from(x).unwrap();
            }
            None => match c {
                'R' => {
                    instructions.push(Instruction::Move(num));
                    instructions.push(Instruction::Rotate(Rotation::Right));
                    num = 0;
                }
                'L' => {
                    instructions.push(Instruction::Move(num));
                    instructions.push(Instruction::Rotate(Rotation::Left));
                    num = 0;
                }
                _ => unreachable!(),
            },
        }
    }
    if num != 0 {
        instructions.push(Instruction::Move(num));
    }

    (map, start, instructions)
}

fn get_min_x(map: &HashMap<Point, char>, y: usize) -> usize {
    map.iter()
        .filter_map(|(p, _)| if p.y == y { Some(p.x) } else { None })
        .min()
        .unwrap()
}

fn get_max_x(map: &HashMap<Point, char>, y: usize) -> usize {
    map.iter()
        .filter_map(|(p, _)| if p.y == y { Some(p.x) } else { None })
        .max()
        .unwrap()
}

fn get_min_y(map: &HashMap<Point, char>, x: usize) -> usize {
    map.iter()
        .filter_map(|(p, _)| if p.x == x { Some(p.y) } else { None })
        .min()
        .unwrap()
}

fn get_max_y(map: &HashMap<Point, char>, x: usize) -> usize {
    map.iter()
        .filter_map(|(p, _)| if p.x == x { Some(p.y) } else { None })
        .max()
        .unwrap()
}

fn get_next_pos(map: &HashMap<Point, char>, pos: &Point, facing: Facing) -> Option<Point> {
    let new_pos = match facing {
        Facing::Right => {
            let new_pos = Point::new(pos.x + 1, pos.y);
            match map.get(&new_pos) {
                Some(_) => new_pos,
                None => Point::new(get_min_x(map, pos.y), pos.y), // wrap around
            }
        }
        Facing::Down => {
            let new_pos = Point::new(pos.x, pos.y + 1);
            match map.get(&new_pos) {
                Some(_) => new_pos,
                None => Point::new(pos.x, get_min_y(map, pos.x)), // wrap around
            }
        }
        Facing::Left => {
            let new_pos = Point::new(pos.x - 1, pos.y);
            match map.get(&new_pos) {
                Some(_) => new_pos,
                None => Point::new(get_max_x(map, pos.y), pos.y), // wrap around
            }
        }
        Facing::Up => {
            let new_pos = Point::new(pos.x, pos.y - 1);
            match map.get(&new_pos) {
                Some(_) => new_pos,
                None => Point::new(pos.x, get_max_y(map, pos.x)), // wrap around
            }
        }
    };
    match map.get(&new_pos) {
        Some(c) => match *c {
            '.' => Some(new_pos),
            '#' => None,
            _ => unreachable!(),
        },
        None => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(usize),
    Rotate(Rotation),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Right, // 90 degrees clockwise
    Left,  // 90 degrees counterclockwise
}

impl Facing {
    fn rotate(self, rotation: Rotation) -> Facing {
        match (self, rotation) {
            (Facing::Right, Rotation::Right) | (Facing::Left, Rotation::Left) => Facing::Down,
            (Facing::Right, Rotation::Left) | (Facing::Left, Rotation::Right) => Facing::Up,
            (Facing::Down, Rotation::Right) | (Facing::Up, Rotation::Left) => Facing::Left,
            (Facing::Down, Rotation::Left) | (Facing::Up, Rotation::Right) => Facing::Right,
        }
    }

    fn to_score(self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day22-sample.txt")), 6032);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day22-sample.txt")), 0);
    }
}
