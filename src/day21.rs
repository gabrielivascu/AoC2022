use std::collections::HashMap;

pub fn solve_1(input: &str) -> i64 {
    let (mut monkeys, mut solved) = build_input(input);
    solved = solver(&mut monkeys, solved);
    *solved.get("root").unwrap()
}

pub fn solve_2(input: &str) -> i64 {
    let (mut monkeys, mut solved) = build_input(input);
    solved.remove("humn");

    solver(&mut monkeys, solved);

    let root = monkeys.iter().find(|m| m.name == "root").unwrap();
    let (mut expected, mut next) = match (root.lhs, root.rhs) {
        (StrOrNum::Str(name), StrOrNum::Num(b)) => (b, name),
        (StrOrNum::Num(a), StrOrNum::Str(name)) => (a, name),
        _ => unreachable!(),
    };
    loop {
        let monkey = monkeys.iter().find(|m| m.name == next).unwrap();
        (expected, next) = match (monkey.lhs, monkey.rhs) {
            // Solve equation for X: $expected = X <op> $rhs
            (StrOrNum::Str(name), StrOrNum::Num(b)) => match monkey.op {
                Operation::Add => (expected - b, name),
                Operation::Subtract => (expected + b, name),
                Operation::Multiply => (expected / b, name),
                Operation::Divide => (expected * b, name),
            },
            // Solve equation for X: $expected = $lhs <op> X
            (StrOrNum::Num(a), StrOrNum::Str(name)) => match monkey.op {
                Operation::Add => (expected - a, name),
                Operation::Subtract => (a - expected, name),
                Operation::Multiply => (expected / a, name),
                Operation::Divide => (a / expected, name),
            },
            _ => unreachable!(),
        };
        if next == "humn" {
            break;
        }
    }

    expected
}

fn solver<'a>(
    monkeys: &'a mut Vec<Monkey>,
    mut solved: HashMap<&'a str, i64>,
) -> HashMap<&'a str, i64> {
    loop {
        let mut unsolved_monkeys = monkeys
            .iter()
            .filter(|m| !solved.contains_key(m.name))
            .copied()
            .collect::<Vec<_>>();
        if unsolved_monkeys.is_empty() {
            break;
        }

        let mut solved_one = false;
        for monkey in &mut unsolved_monkeys {
            let lhs = match monkey.lhs {
                StrOrNum::Num(x) => Some(x),
                StrOrNum::Str(name) => solved.get(name).copied(),
            };
            let rhs = match monkey.rhs {
                StrOrNum::Num(x) => Some(x),
                StrOrNum::Str(name) => solved.get(name).copied(),
            };

            match (lhs, rhs) {
                (Some(a), Some(b)) => {
                    let result = match monkey.op {
                        Operation::Add => a + b,
                        Operation::Subtract => a - b,
                        Operation::Multiply => a * b,
                        Operation::Divide => a / b,
                    };
                    solved.insert(monkey.name, result);
                    solved_one = true;
                }
                (Some(a), None) => {
                    monkey.lhs = StrOrNum::Num(a);
                }
                (None, Some(b)) => {
                    monkey.rhs = StrOrNum::Num(b);
                }
                (None, None) => {}
            }
        }

        // Make sure to update the list BEFORE breaking the loop.
        *monkeys = unsolved_monkeys;

        if !solved_one {
            break;
        }
    }
    solved
}

fn build_input(input: &str) -> (Vec<Monkey>, HashMap<&str, i64>) {
    let mut monkeys = vec![];
    let mut solved = HashMap::new();

    for line in input.lines() {
        let parts = line.split_once(": ").unwrap();
        if let Ok(value) = parts.1.parse::<i64>() {
            solved.insert(parts.0, value);
        } else {
            let mut ops = parts.1.split(' ');
            let lhs = ops.next().unwrap();
            let op = ops.next().unwrap();
            let rhs = ops.next().unwrap();

            let op = match op {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => unreachable!(),
            };

            monkeys.push(Monkey {
                name: parts.0,
                lhs: StrOrNum::Str(lhs),
                rhs: StrOrNum::Str(rhs),
                op,
            });
        }
    }

    (monkeys, solved)
}

#[derive(Debug, Clone, Copy)]
struct Monkey<'a> {
    name: &'a str,
    lhs: StrOrNum<'a>,
    rhs: StrOrNum<'a>,
    op: Operation,
}

#[derive(Debug, Clone, Copy)]
enum StrOrNum<'a> {
    Str(&'a str),
    Num(i64),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day21-sample.txt")), 152);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day21-sample.txt")), 301);
    }
}
