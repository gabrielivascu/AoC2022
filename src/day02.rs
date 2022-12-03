pub fn solve_1() -> i64 {
    include_str!("../input/day02.txt")
        .lines()
        .fold(0, |acc, line| {
            let mut tokens = line.split(' ');
            let (lhs, rhs) = (tokens.next().unwrap(), tokens.next().unwrap());
            let score = match (lhs, rhs) {
                ("A", "X") => 3 + 1, // draw with rock
                ("B", "Y") => 3 + 2, // draw with paper
                ("C", "Z") => 3 + 3, // draw with scissors
                ("C", "X") => 6 + 1, // win with rock
                ("A", "Y") => 6 + 2, // win with paper
                ("B", "Z") => 6 + 3, // win with scissors
                (_, "X") => 1,       // loss with rock
                (_, "Y") => 2,       // loss with paper
                (_, "Z") => 3,       // loss with scissors
                _ => 0,
            };
            acc + score
        })
}

pub fn solve_2() -> i64 {
    include_str!("../input/day02.txt")
        .lines()
        .fold(0, |acc, line| {
            let mut tokens = line.split(' ');
            let (lhs, rhs) = (tokens.next().unwrap(), tokens.next().unwrap());
            let score = match (lhs, rhs) {
                ("A", "Y") => 3 + 1, // draw with rock
                ("B", "Y") => 3 + 2, // draw with paper
                ("C", "Y") => 3 + 3, // draw with scissors
                ("A", "X") => 3,     // loss with scissors
                ("B", "X") => 1,     // loss with rock
                ("C", "X") => 2,     // loss with paper
                ("A", "Z") => 6 + 2, // win with paper
                ("B", "Z") => 6 + 3, // win with scissors
                ("C", "Z") => 6 + 1, // win with rock
                _ => 0,
            };
            acc + score
        })
}
