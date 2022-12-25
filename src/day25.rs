pub fn solve(input: &str) -> String {
    decimal2snafu(input.lines().map(snafu2decimal).sum())
}

fn snafu2decimal(snafu: &str) -> i64 {
    snafu.chars().rev().enumerate().fold(0_i64, |acc, (i, c)| {
        let base = 5_i64.pow(u32::try_from(i).unwrap());
        acc + match c {
            '0' => 0,
            '1' => base,
            '2' => base * 2,
            '-' => -base,
            '=' => -base * 2,
            _ => unreachable!(),
        }
    })
}

fn decimal2snafu(mut decimal: i64) -> String {
    let mut res = String::new();
    while decimal > 0 {
        match (decimal + 2) % 5 {
            0 => res.push('='),
            1 => res.push('-'),
            2 => res.push('0'),
            3 => res.push('1'),
            4 => res.push('2'),
            _ => unreachable!(),
        }
        decimal = (decimal + 2) / 5;
    }
    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve(include_str!("../input/day25-sample.txt")),
            String::from("2=-1=0")
        );
    }
}
