use itertools::iproduct;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub fn solve_1(input: &str) -> usize {
    solver(&build_input_cubes(input))
}

fn solver(cubes: &[Point]) -> usize {
    let mut all_faces = HashMap::new();
    for cube in cubes {
        let mut faces = cube.cube_faces();
        for face in &mut faces {
            face.sort();
        }
        for face in faces {
            let count = all_faces.entry(face).or_insert(0_usize);
            *count += 1;
        }
    }
    all_faces.values().filter(|&val| *val == 1).count()
}

pub fn solve_2(input: &str) -> usize {
    let mut lava_cubes = build_input_cubes(input);
    lava_cubes.sort();

    let (x_min, x_max) = (
        lava_cubes.iter().map(|p| p.x).min().unwrap() - 1,
        lava_cubes.iter().map(|p| p.x).max().unwrap() + 2,
    );
    let (y_min, y_max) = (
        lava_cubes.iter().map(|p| p.y).min().unwrap() - 1,
        lava_cubes.iter().map(|p| p.y).max().unwrap() + 2,
    );
    let (z_min, z_max) = (
        lava_cubes.iter().map(|p| p.z).min().unwrap() - 1,
        lava_cubes.iter().map(|p| p.z).max().unwrap() + 2,
    );

    // Build a map of air pockets. Initially all of them are not filled.
    let mut air_pockets: HashMap<Point, bool> = HashMap::new();
    for (x, y, z) in iproduct!(x_min..x_max, y_min..y_max, z_min..z_max)
        .filter(|(x, y, z)| lava_cubes.binary_search(&Point::new(*x, *y, *z)).is_err())
    {
        air_pockets.insert(Point::new(x, y, z), false);
    }

    // Run a BFS and flood-fill the air pockets.
    let mut queue = VecDeque::new();
    queue.push_back(Point::new(x_min, y_min, z_min));
    while !queue.is_empty() {
        let pocket = queue.pop_front().unwrap();
        air_pockets
            .entry(pocket)
            .and_modify(|filled| *filled = true); // mark as filled
        for neigh in pocket.neighbors() {
            if let Some(filled) = air_pockets.get(&neigh) {
                if !filled && !queue.contains(&neigh) {
                    queue.push_back(neigh);
                }
            }
        }
    }

    let air_pockets_unfilled = air_pockets
        .iter()
        .filter(|(_, &filled)| !filled)
        .map(|(pocket, _)| *pocket)
        .collect::<Vec<_>>();

    solver(&lava_cubes) - solver(&air_pockets_unfilled)
}

fn build_input_cubes(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|n| n.parse().unwrap());
            Point::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn neighbors(&self) -> [Point; 6] {
        [
            Point::new(self.x - 1, self.y, self.z),
            Point::new(self.x + 1, self.y, self.z),
            Point::new(self.x, self.y - 1, self.z),
            Point::new(self.x, self.y + 1, self.z),
            Point::new(self.x, self.y, self.z - 1),
            Point::new(self.x, self.y, self.z + 1),
        ]
    }

    fn cube_faces(&self) -> [Vec<Point>; 6] {
        let corners = [
            Point::new(self.x, self.y + 1, self.z + 1),
            Point::new(self.x + 1, self.y + 1, self.z + 1),
            Point::new(self.x + 1, self.y + 1, self.z),
            Point::new(self.x, self.y + 1, self.z),
            Point::new(self.x, self.y, self.z + 1),
            Point::new(self.x + 1, self.y, self.z + 1),
            Point::new(self.x + 1, self.y, self.z),
            Point::new(self.x, self.y, self.z),
        ];
        [
            vec![corners[0], corners[1], corners[2], corners[3]],
            vec![corners[4], corners[5], corners[6], corners[7]],
            vec![corners[4], corners[5], corners[1], corners[0]],
            vec![corners[7], corners[6], corners[2], corners[3]],
            vec![corners[7], corners[4], corners[0], corners[3]],
            vec![corners[1], corners[5], corners[6], corners[2]],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day18-sample.txt")), 64);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day18-sample.txt")), 58);
    }
}
