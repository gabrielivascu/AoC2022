use std::collections::HashMap;

pub fn solve_1(input: &str) -> i64 {
    let regmap = build_register_cycle_map(input);
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|x| x * regmap.get(x).unwrap())
        .sum()
}

pub fn solve_2(input: &str) -> String {
    let regmap = build_register_cycle_map(input);
    let mut display = String::new();

    for idx in 0..240 {
        let regval = *regmap.get(&(idx + 1)).unwrap();
        if (regval - 1..=regval + 1).contains(&(idx % 40)) {
            display.push('#');
        } else {
            display.push('.');
        }
        if (idx + 1) % 40 == 0 && idx != 239 {
            display.push('\n');
        }
    }

    display
}

fn build_register_cycle_map(input: &str) -> HashMap<i64, i64> {
    let mut regval = 1;
    let mut cycle = 0;
    let mut regmap = HashMap::new();

    for line in input.lines() {
        cycle += 1;
        regmap.insert(cycle, regval);
        if line.starts_with("addx") {
            cycle += 1;
            regmap.insert(cycle, regval);
            regval += line[5..].parse::<i64>().unwrap();
        }
    }

    regmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day10-sample.txt")), 13140);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(include_str!("../input/day10-sample.txt")),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
