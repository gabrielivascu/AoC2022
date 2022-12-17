use std::collections::HashMap;

pub fn solve_1(input: &str) -> usize {
    let mut grid = build_input_grid(input);
    let source = Point::new(500, 0);

    let mut stopped = false;
    while !stopped {
        let mut curr = source;
        loop {
            if curr.y > grid.pmax.y {
                // Reached abyss, stop.
                stopped = true;
                break;
            }

            let bottom = Point::new(curr.x, curr.y + 1);
            let bottom_left = Point::new(curr.x - 1, curr.y + 1);
            let bottom_right = Point::new(curr.x + 1, curr.y + 1);

            if !grid.map.contains_key(&bottom) {
                // Bottom is available.
                curr = bottom;
            } else if !grid.map.contains_key(&bottom_left) {
                // Bottom-left if available.
                curr = bottom_left;
            } else if !grid.map.contains_key(&bottom_right) {
                // Bottom-right if available.
                curr = bottom_right;
            } else {
                // Nowhere to move.
                grid.map.insert(curr, 'o');
                break;
            }
        }
    }

    grid.map.values().filter(|&x| *x == 'o').count()
}

pub fn solve_2(input: &str) -> usize {
    let mut grid = build_input_grid(input);
    let source = Point::new(500, 0);

    let mut stopped = false;
    while !stopped {
        let mut curr = source;
        loop {
            if curr.y == grid.pmax.y + 1 {
                // Reached bottom line, nowhere to go.
                grid.map.insert(curr, 'o');
                break;
            }

            let bottom = Point::new(curr.x, curr.y + 1);
            let bottom_left = Point::new(curr.x - 1, curr.y + 1);
            let bottom_right = Point::new(curr.x + 1, curr.y + 1);

            if !grid.map.contains_key(&bottom) {
                // Bottom is available.
                curr = bottom;
            } else if !grid.map.contains_key(&bottom_left) {
                // Bottom-left if available.
                curr = bottom_left;
            } else if !grid.map.contains_key(&bottom_right) {
                // Bottom-right if available.
                curr = bottom_right;
            } else {
                // Nowhere to move.
                grid.map.insert(curr, 'o');
                if curr == source {
                    stopped = true;
                }
                break;
            }
        }
    }

    grid.map.values().filter(|&x| *x == 'o').count()
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

#[derive(Debug)]
struct Grid {
    map: HashMap<Point, char>,
    pmax: Point,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map: HashMap::new(),
            pmax: Point::new(0, 0),
        }
    }
}

fn build_input_grid(input: &str) -> Grid {
    let mut grid = Grid::new();
    for line in input.lines() {
        let points = line
            .split(" -> ")
            .map(|point| {
                let (x, y) = point.split_once(',').unwrap();
                Point::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<_>>();
        for i in 0..points.len() - 1 {
            let (p1, p2) = (points[i], points[i + 1]);
            grid.pmax.x = grid.pmax.x.max(p1.x.max(p2.x));
            grid.pmax.y = grid.pmax.y.max(p1.y.max(p2.y));

            if p1.x == p2.x {
                // x is constant, vary y.
                for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
                    grid.map.insert(Point::new(p1.x, y), '#');
                }
            } else if p1.y == p2.y {
                // y is constant, vary x.
                for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
                    grid.map.insert(Point::new(x, p1.y), '#');
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day14-sample.txt")), 24);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day14-sample.txt")), 93);
    }
}
