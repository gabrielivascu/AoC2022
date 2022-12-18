use std::collections::HashMap;

pub fn solve_1(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|n| n.parse::<usize>().unwrap());
            Cube::new(Point::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            ))
        })
        .collect::<Vec<_>>();

    let mut all_faces = HashMap::new();
    for cube in &cubes {
        for face in cube.faces() {
            let count = all_faces.entry(face).or_insert(0_usize);
            *count += 1;
        }
    }

    all_faces.values().filter(|&val| *val == 1).count()
}

pub fn solve_2(_input: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Point {
        Point { x, y, z }
    }
}

#[derive(Debug)]
struct Cube {
    corners: [Point; 8],
}

impl Cube {
    fn new(origin: Point) -> Cube {
        let corners = [
            Point::new(origin.x, origin.y + 1, origin.z + 1),
            Point::new(origin.x + 1, origin.y + 1, origin.z + 1),
            Point::new(origin.x + 1, origin.y + 1, origin.z),
            Point::new(origin.x, origin.y + 1, origin.z),
            Point::new(origin.x, origin.y, origin.z + 1),
            Point::new(origin.x + 1, origin.y, origin.z + 1),
            Point::new(origin.x + 1, origin.y, origin.z),
            Point::new(origin.x, origin.y, origin.z),
        ];
        Cube { corners }
    }

    fn faces(&self) -> [Vec<Point>; 6] {
        let mut faces = [
            vec![
                self.corners[0],
                self.corners[1],
                self.corners[2],
                self.corners[3],
            ],
            vec![
                self.corners[4],
                self.corners[5],
                self.corners[6],
                self.corners[7],
            ],
            vec![
                self.corners[4],
                self.corners[5],
                self.corners[1],
                self.corners[0],
            ],
            vec![
                self.corners[7],
                self.corners[6],
                self.corners[2],
                self.corners[3],
            ],
            vec![
                self.corners[7],
                self.corners[4],
                self.corners[0],
                self.corners[3],
            ],
            vec![
                self.corners[1],
                self.corners[5],
                self.corners[6],
                self.corners[2],
            ],
        ];
        for face in &mut faces {
            face.sort();
        }
        faces
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
        assert_eq!(solve_2(include_str!("../input/day18-sample.txt")), 0);
    }
}
