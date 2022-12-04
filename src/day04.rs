pub fn solve_1() -> usize {
    include_str!("../input/day04.txt")
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

pub fn solve_2() -> usize {
    include_str!("../input/day04.txt")
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
