fn solver_1(data: &[(Sensor, Beacon)], p_min: Point, p_max: Point, y_axis: i32) -> usize {
    let mut count = 0;

    // Search the base line from [min X, max X].
    for x in p_min.x..=p_max.x {
        let p = Point::new(x, y_axis);
        if data
            .iter()
            .any(|(s, b)| p != *s && p != *b && point_violates(p, *s, *b))
        {
            count += 1;
        }
    }
    // Search from (min X, -infinity) and stop at the first position that is valid.
    for x in (i32::MIN..p_min.x).rev() {
        let p = Point::new(x, y_axis);
        if data
            .iter()
            .any(|(s, b)| p != *s && p != *b && point_violates(p, *s, *b))
        {
            count += 1;
        } else {
            break;
        }
    }
    // Search from (max X, +infinity) and stop at the first position that is valid.
    for x in p_max.x + 1..=i32::MAX {
        let p = Point::new(x, y_axis);
        if data
            .iter()
            .any(|(s, b)| p != *s && p != *b && point_violates(p, *s, *b))
        {
            count += 1;
        } else {
            break;
        }
    }

    count
}

pub fn solve_1(input: &str) -> usize {
    let (data, p_min, p_max) = build_input_data(input);
    solver_1(&data, p_min, p_max, 2_000_000)
}

fn solver_2(data: &Vec<(Sensor, Beacon)>, width: i32) -> i64 {
    let uncovered = compute_uncovered(data, &Rectangle::new(Point::new(0, 0), width, width));
    assert!(uncovered.len() == 1, "bug: more than one solution");
    let beacon = uncovered[0];
    assert!(
        beacon.width == 1 && beacon.height == 1,
        "bug: not an 1x1 rectangle"
    );
    i64::from(beacon.point.x) * 4_000_000 + i64::from(beacon.point.y)
}

pub fn solve_2(input: &str) -> i64 {
    let (data, _, _) = build_input_data(input);
    solver_2(&data, 4_000_001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (data, p_min, p_max) = build_input_data(include_str!("../input/day15-sample.txt"));
        assert_eq!(solver_1(&data, p_min, p_max, 10), 26);
    }

    #[test]
    fn test_2() {
        let (data, _, _) = build_input_data(include_str!("../input/day15-sample.txt"));
        assert_eq!(solver_2(&data, 21), 56000011);
    }
}

fn build_input_data(input: &str) -> (Vec<(Sensor, Beacon)>, Point, Point) {
    let (mut x_min, mut y_min) = (i32::MAX, i32::MAX);
    let (mut x_max, mut y_max) = (i32::MIN, i32::MIN);

    let data = input
        .lines()
        .map(|line| {
            let (s_part, b_part) = line.split_once(": ").unwrap();
            let (sx, sy) = s_part.split_once(", ").unwrap();
            let (sx, sy) = (
                sx[12..].parse::<i32>().unwrap(),
                sy[2..].parse::<i32>().unwrap(),
            );
            let (bx, by) = b_part.split_once(", ").unwrap();
            let (bx, by) = (
                bx[23..].parse::<i32>().unwrap(),
                by[2..].parse::<i32>().unwrap(),
            );

            (x_min, y_min) = (x_min.min(sx.min(bx)), y_min.min(sy.min(by)));
            (x_max, y_max) = (x_max.max(sx.max(bx)), y_max.max(sy.max(by)));

            (Sensor::new(sx, sy), Beacon::new(bx, by))
        })
        .collect::<Vec<_>>();

    (data, Point::new(x_min, y_min), Point::new(x_max, y_max))
}

fn point_violates(point: Point, sensor: Sensor, beacon: Beacon) -> bool {
    let dist_b = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
    let dist_p = (sensor.x - point.x).abs() + (sensor.y - point.y).abs();
    dist_p <= dist_b
}

fn compute_uncovered(data: &Vec<(Sensor, Beacon)>, rectangle: &Rectangle) -> Vec<Rectangle> {
    // No point to bother with "empty" rectangles.
    if rectangle.width == 0 || rectangle.height == 0 {
        return vec![];
    }
    // If all corners violate at least one sensor, then the whole rectangle is covered.
    if data.iter().any(|(s, b)| {
        rectangle
            .corners()
            .iter()
            .all(|corner| point_violates(*corner, *s, *b))
    }) {
        return vec![];
    }
    // An 1x1 rectangle that doesn't violate any sensor => found the hidden beacon.
    if rectangle.width == 1 && rectangle.height == 1 {
        return vec![*rectangle];
    }
    // Recursively check the inner rectangles.
    let mut res = vec![];
    for inner in rectangle.split4() {
        res.append(&mut compute_uncovered(data, &inner));
    }
    res
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

type Sensor = Point;
type Beacon = Point;

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    point: Point,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(point: Point, width: i32, height: i32) -> Rectangle {
        Rectangle {
            point,
            width,
            height,
        }
    }

    fn corners(&self) -> Vec<Point> {
        vec![
            Point::new(self.point.x, self.point.y),
            Point::new(self.point.x + self.width - 1, self.point.y),
            Point::new(
                self.point.x + self.width - 1,
                self.point.y + self.height - 1,
            ),
            Point::new(self.point.x, self.point.y + self.height - 1),
        ]
    }

    /// Split into four smaller inner rectangles.
    fn split4(&self) -> Vec<Rectangle> {
        let (width1, width2) = (self.width / 2, self.width - self.width / 2);
        let (height1, height2) = (self.height / 2, self.height - self.height / 2);
        vec![
            Rectangle::new(Point::new(self.point.x, self.point.y), width1, height1),
            Rectangle::new(
                Point::new(self.point.x + width1, self.point.y),
                width2,
                height1,
            ),
            Rectangle::new(
                Point::new(self.point.x, self.point.y + height1),
                width1,
                height2,
            ),
            Rectangle::new(
                Point::new(self.point.x + width1, self.point.y + height1),
                width2,
                height2,
            ),
        ]
    }
}
