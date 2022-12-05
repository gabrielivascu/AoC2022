pub fn solve_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut parts = line.split(',');
            let (seg1, seg2) = (parts.next().unwrap(), parts.next().unwrap());

            let mut seg1_parts = seg1.split('-');
            let (a1, b1) = (seg1_parts.next().unwrap(), seg1_parts.next().unwrap());
            let (a1, b1) = (a1.parse::<u32>().unwrap(), b1.parse::<u32>().unwrap());

            let mut seg2_parts = seg2.split('-');
            let (a2, b2) = (seg2_parts.next().unwrap(), seg2_parts.next().unwrap());
            let (a2, b2) = (a2.parse::<u32>().unwrap(), b2.parse::<u32>().unwrap());

            // a2    a1    b1    b2
            // |     |_____|     |
            // |_________________|
            //
            // (or segments inverted)
            (a2 <= a1 && b1 <= b2) || (a1 <= a2 && b2 <= b1)
        })
        .count()
}

pub fn solve_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut parts = line.split(',');
            let (seg1, seg2) = (parts.next().unwrap(), parts.next().unwrap());

            let mut seg1_parts = seg1.split('-');
            let (a1, b1) = (seg1_parts.next().unwrap(), seg1_parts.next().unwrap());
            let (a1, b1) = (a1.parse::<u32>().unwrap(), b1.parse::<u32>().unwrap());

            let mut seg2_parts = seg2.split('-');
            let (a2, b2) = (seg2_parts.next().unwrap(), seg2_parts.next().unwrap());
            let (a2, b2) = (a2.parse::<u32>().unwrap(), b2.parse::<u32>().unwrap());

            // Same as above, plus
            //
            // a1    a2    b1    b2
            // |_____|_____|     |
            //       |___________|
            //
            // (or segments inverted)
            (a2 <= a1 && b1 <= b2)
                || (a1 <= a2 && b2 <= b1)
                || (a1 <= a2 && a2 <= b1 && b1 <= b2)
                || (a2 <= a1 && a1 <= b2 && b2 <= b1)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day04-sample.txt")), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day04-sample.txt")), 4);
    }
}
