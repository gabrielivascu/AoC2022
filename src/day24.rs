use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve_1(input: &str) -> i32 {
    let (map, blizzards) = build_input_map(input);
    solver(&map, &blizzards, map.start, map.end, 0)
}

pub fn solve_2(input: &str) -> i32 {
    let (map, blizzards) = build_input_map(input);
    solver(
        &map,
        &blizzards,
        map.start,
        map.end,
        solver(
            &map,
            &blizzards,
            map.end,
            map.start,
            solver(&map, &blizzards, map.start, map.end, 0),
        ),
    )
}

fn solver(map: &Map, blizzards: &[Blizzard], start: Point, end: Point, start_minute: i32) -> i32 {
    let start = (start, start_minute);

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut visited = HashSet::new();
    visited.insert(start);

    // Map minute to blizzard positions.
    let mut blizzards_cache: HashMap<i32, HashSet<Point>> = HashMap::new();

    while !queue.is_empty() {
        let (pos, minute) = queue.pop_front().unwrap();
        if pos == end {
            return minute;
        }

        // Compute blizzard positions for the next minute and cache them.
        blizzards_cache.entry(minute + 1).or_insert_with(|| {
            blizzards
                .iter()
                .map(|b| b.r#move(map, minute + 1))
                .collect()
        });

        let mut next_moves = map.neighbors(pos);
        next_moves.push(pos); // wait on the same spot

        for next in next_moves.iter().map(|pos| (*pos, minute + 1)) {
            if !visited.contains(&next) && !blizzards_cache.get(&next.1).unwrap().contains(&next.0)
            {
                queue.push_back(next);
                visited.insert(next);
            }
        }
    }

    -1
}

fn build_input_map(input: &str) -> (Map, Vec<Blizzard>) {
    let (width, height) = (
        i32::try_from(input.lines().next().unwrap().len()).unwrap(),
        i32::try_from(input.lines().count()).unwrap(),
    );
    let start = Point::new(i32::try_from(input.find('.').unwrap()).unwrap(), 0);
    let end = Point::new(
        i32::try_from(input.lines().next_back().unwrap().find('.').unwrap()).unwrap(),
        height - 1,
    );

    let mut blizzards = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => blizzards.push(Blizzard::new(
                    Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                    Direction::Right,
                )),
                '<' => blizzards.push(Blizzard::new(
                    Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                    Direction::Left,
                )),
                '^' => blizzards.push(Blizzard::new(
                    Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                    Direction::Up,
                )),
                'v' => blizzards.push(Blizzard::new(
                    Point::new(i32::try_from(x).unwrap(), i32::try_from(y).unwrap()),
                    Direction::Down,
                )),
                _ => {}
            }
        }
    }

    (Map::new(start, end, width, height), blizzards)
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Blizzard {
    start: Point,
    dir: Direction,
}

impl Blizzard {
    fn new(pos: Point, dir: Direction) -> Blizzard {
        Self { start: pos, dir }
    }

    fn r#move(&self, map: &Map, minute: i32) -> Point {
        let steps = match self.dir {
            Direction::Up | Direction::Down => minute % (map.height - 2),
            Direction::Left | Direction::Right => minute % (map.width - 2),
        };
        match self.dir {
            Direction::Up => {
                let mut new_y = self.start.y;
                for _ in 0..steps {
                    new_y = if new_y == 1 {
                        map.height - 2
                    } else {
                        new_y - 1
                    };
                }
                Point::new(self.start.x, new_y)
            }
            Direction::Down => {
                let mut new_y = self.start.y;
                for _ in 0..steps {
                    new_y = if new_y == map.height - 2 {
                        1
                    } else {
                        new_y + 1
                    };
                }
                Point::new(self.start.x, new_y)
            }
            Direction::Left => {
                let mut new_x = self.start.x;
                for _ in 0..steps {
                    new_x = if new_x == 1 { map.width - 2 } else { new_x - 1 };
                }
                Point::new(new_x, self.start.y)
            }
            Direction::Right => {
                let mut new_x = self.start.x;
                for _ in 0..steps {
                    new_x = if new_x == map.width - 2 { 1 } else { new_x + 1 };
                }
                Point::new(new_x, self.start.y)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Map {
    start: Point,
    end: Point,
    width: i32,
    height: i32,
}

impl Map {
    fn new(start: Point, end: Point, width: i32, height: i32) -> Map {
        Self {
            start,
            end,
            width,
            height,
        }
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = vec![
            Point::new(point.x, point.y - 1),
            Point::new(point.x, point.y + 1),
            Point::new(point.x - 1, point.y),
            Point::new(point.x + 1, point.y),
        ];
        // Skip borders, but allow (start), (end).
        neighbors.retain(|p| {
            (*p == self.start || *p == self.end)
                || ((1..self.width - 1).contains(&p.x) && (1..self.height - 1).contains(&p.y))
        });
        neighbors
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day24-sample.txt")), 18);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day24-sample.txt")), 54);
    }
}
