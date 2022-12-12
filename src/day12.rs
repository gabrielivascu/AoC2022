use itertools::iproduct;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Grid {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            map: Vec::new(),
            height: 0,
            width: 0,
        }
    }

    fn get_valid_neighbors(&self, (y, x): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        // Nasty way to bypass 'attempt to subtract with overflow' panic.
        if y > 0 && (0..=self.map[y][x] + 1).contains(&self.map[y - 1][x]) {
            neighbors.push((y - 1, x));
        }
        if y < self.height - 1 && (0..=self.map[y][x] + 1).contains(&self.map[y + 1][x]) {
            neighbors.push((y + 1, x));
        }
        if x > 0 && (0..=self.map[y][x] + 1).contains(&self.map[y][x - 1]) {
            neighbors.push((y, x - 1));
        }
        if x < self.width - 1 && (0..=self.map[y][x] + 1).contains(&self.map[y][x + 1]) {
            neighbors.push((y, x + 1));
        }
        neighbors
    }
}

fn build_input_grid(input: &str) -> (Grid, (usize, usize), (usize, usize)) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut grid = Grid::new();
    for line in input.lines().map(str::as_bytes) {
        grid.width = line.len();
        let mut v = Vec::new();
        for (x, val) in line.iter().enumerate() {
            if "S".as_bytes()[0] == *val {
                start = (grid.height, x);
                v.push("a".as_bytes()[0]);
            } else if "E".as_bytes()[0] == *val {
                end = (grid.height, x);
                v.push("z".as_bytes()[0]);
            } else {
                v.push(*val);
            }
        }
        grid.map.push(v);
        grid.height += 1;
    }
    (grid, start, end)
}

fn dijkstra_shortest_path(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> u32 {
    // Keep a sorted set (priority queue) by distance to the start node.
    let mut pqueue = BTreeSet::new();
    pqueue.insert((0, start));
    let mut distance = HashMap::new();
    distance.entry(start).or_insert(0);

    while !pqueue.is_empty() {
        let closest = *pqueue.iter().next().unwrap();
        pqueue.remove(&closest);
        let (dist_u, u) = closest;
        for v in grid.get_valid_neighbors(u) {
            let dist_v = *distance.entry(v).or_insert(u32::MAX);
            if dist_v > dist_u + 1 {
                if dist_v != u32::MAX {
                    pqueue.remove(&(dist_v, v));
                }
                distance.entry(v).and_modify(|d| *d = dist_u + 1);
                pqueue.insert((dist_u + 1, v));
            }
        }
    }
    *distance.get(&end).unwrap_or(&u32::MAX)
}

pub fn solve_1(input: &str) -> u32 {
    let (grid, start, end) = build_input_grid(input);
    dijkstra_shortest_path(&grid, start, end)
}

pub fn solve_2(input: &str) -> u32 {
    let (grid, _, end) = build_input_grid(input);
    let mut minpath = u32::MAX;
    for (y, x) in iproduct!(0..grid.height, 0..grid.width) {
        if "a".as_bytes()[0] == grid.map[y][x] {
            minpath = minpath.min(dijkstra_shortest_path(&grid, (y, x), end));
        }
    }
    minpath
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day12-sample.txt")), 31);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day12-sample.txt")), 29);
    }
}
