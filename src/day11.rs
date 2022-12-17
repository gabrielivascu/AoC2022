use itertools::Itertools;
use std::cell::RefCell;

pub fn solve_1(input: &str) -> usize {
    let monkeys = build_monkeys(input);
    solver(&monkeys, 20, Some(3))
}

pub fn solve_2(input: &str) -> usize {
    let monkeys = build_monkeys(input);
    solver(&monkeys, 10000, None)
}

#[derive(Debug, Clone)]
enum Operation {
    None,
    Square,
    Add(i64),
    Multiply(i64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: (i64, usize, usize),
    count: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: vec![],
            op: Operation::None,
            test: (0, 0, 0),
            count: 0,
        }
    }
}

fn build_monkeys(input: &str) -> Vec<RefCell<Monkey>> {
    let mut monkeys = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("Monkey") {
            monkeys.push(RefCell::new(Monkey::new()));
        } else if line.starts_with("Starting") {
            monkeys.last_mut().unwrap().borrow_mut().items = line[16..]
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        } else if line.starts_with("Operation") {
            let op = &line[21..22];
            let rhs = &line[23..];
            match (op, rhs) {
                ("+", _) => {
                    monkeys.last_mut().unwrap().borrow_mut().op =
                        Operation::Add(rhs.parse().unwrap());
                }
                ("*", "old") => {
                    monkeys.last_mut().unwrap().borrow_mut().op = Operation::Square;
                }
                ("*", _) => {
                    monkeys.last_mut().unwrap().borrow_mut().op =
                        Operation::Multiply(rhs.parse().unwrap());
                }
                (_, _) => unreachable!(),
            }
        } else if line.starts_with("Test") {
            monkeys.last_mut().unwrap().borrow_mut().test.0 =
                line.rsplit_once(' ').unwrap().1.parse::<i64>().unwrap();
        } else if line.starts_with("If true") {
            monkeys.last_mut().unwrap().borrow_mut().test.1 =
                line.rsplit_once(' ').unwrap().1.parse::<usize>().unwrap();
        } else if line.starts_with("If false") {
            monkeys.last_mut().unwrap().borrow_mut().test.2 =
                line.rsplit_once(' ').unwrap().1.parse::<usize>().unwrap();
        }
    }
    monkeys
}

fn solver(monkeys: &Vec<RefCell<Monkey>>, rounds: usize, divider: Option<i64>) -> usize {
    let mut lcm = 1;
    for monkey in monkeys {
        lcm = num_integer::lcm(lcm, monkey.borrow().test.0);
    }

    for _ in 0..rounds {
        for monkey in monkeys {
            let mut monkey = monkey.borrow_mut();
            monkey.items.iter().for_each(|item| {
                let mut new_item = match monkey.op {
                    Operation::Square => item * item,
                    Operation::Add(x) => item + x,
                    Operation::Multiply(x) => item * x,
                    Operation::None => unreachable!(),
                };
                new_item = match divider {
                    Some(d) => new_item / d,
                    None => new_item % lcm,
                };
                if new_item % monkey.test.0 == 0 {
                    monkeys[monkey.test.1].borrow_mut().items.push(new_item);
                } else {
                    monkeys[monkey.test.2].borrow_mut().items.push(new_item);
                }
            });
            monkey.count += monkey.items.len();
            monkey.items.clear();
        }
    }

    monkeys
        .iter()
        .map(|x| x.borrow().count)
        .sorted_by(|a, b| Ord::cmp(b, a))
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day11-sample.txt")), 10605);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            solve_2(include_str!("../input/day11-sample.txt")),
            2713310158
        );
    }
}
