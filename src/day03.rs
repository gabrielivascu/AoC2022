pub fn solve_1() -> u32 {
    include_str!("../input/03.txt")
        .lines()
        .map(|x| {
            assert!(x.len() % 2 == 0, "line '{x}' has odd length, expected even");
            let lower = &x[..(x.len() / 2)];
            let upper = &x[(x.len() / 2)..];
            for c in lower.chars() {
                if upper.contains(c) {
                    if c.is_ascii_lowercase() {
                        return c as u32 - 'a' as u32 + 1;
                    } else if c.is_ascii_uppercase() {
                        return c as u32 - 'A' as u32 + 27;
                    }
                    panic!("not an ascii character: '{c}'")
                }
            }
            0
        })
        .sum()
}

pub fn solve_2() -> u32 {
    let lines = include_str!("../input/03.txt").lines().collect::<Vec<_>>();
    assert!(
        lines.len() % 3 == 0,
        "number of lines is not a multiple of 3"
    );

    let mut sum = 0;

    for i in (0..lines.len()).step_by(3) {
        let (first, second, third) = (lines[i], lines[i + 1], lines[i + 2]);
        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                if c.is_ascii_lowercase() {
                    sum += c as u32 - 'a' as u32 + 1;
                } else if c.is_ascii_uppercase() {
                    sum += c as u32 - 'A' as u32 + 27;
                } else {
                    panic!("not an ascii character: '{c}'")
                }
                break;
            }
        }
    }

    sum
}
