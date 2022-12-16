use itertools::iproduct;
use std::collections::{HashMap, HashSet};

pub fn solve_1(input: &str) -> usize {
    let graph = build_input_graph(input);

    let mut states = HashMap::new();
    states.insert(Progress::new("AA", 0), State::new(HashSet::new(), 0));

    for minute in 1..30 {
        let mut new_states = HashMap::new();
        for (p, s) in &states {
            // Build the next states by moving to each neighboring valve.
            for neighbor in &graph.get(p.valve).unwrap().neighbors {
                // Move to the new valve and consume a minute. No new valve is
                // opened. No progress to the total release is made.
                new_states.insert(
                    Progress::new(neighbor, p.release),
                    State::new(s.opened.clone(), s.minute + 1),
                );
            }
            // Open the current valve if not already opened and it's worth opening.
            let flow = graph.get(p.valve).unwrap().flow;
            if !s.opened.contains(p.valve) && flow > 0 {
                let mut new_opened = s.opened.clone();
                new_opened.insert(p.valve);
                new_states.insert(
                    Progress::new(p.valve, p.release + flow * (30 - minute)),
                    State::new(new_opened, s.minute + 1),
                );
            }
        }
        states = new_states;
    }

    states.keys().map(|p| p.release).max().unwrap()
}

pub fn solve_2(input: &str) -> usize {
    let graph = build_input_graph(input);

    let mut states = HashMap::new();
    states.insert(
        ProgressTwo::new("AA", "AA", 0),
        State::new(HashSet::new(), 0),
    );

    for minute in 1..=26 {
        let mut new_states = HashMap::new();
        for (p, s) in &states {
            // Compound each possible state: both stay in their previous positions,
            // I stay and the elephant moves, I move and the elephant stays.
            for (my_valve, elephant_valve) in iproduct!(
                graph
                    .get(p.me)
                    .unwrap()
                    .neighbors
                    .iter()
                    .chain([p.me].iter()),
                graph
                    .get(p.elephant)
                    .unwrap()
                    .neighbors
                    .iter()
                    .chain([p.elephant].iter())
            ) {
                let mut new_state = State::new(s.opened.clone(), s.minute + 1);
                let mut new_release = p.release;

                let my_flow = graph.get(my_valve).unwrap().flow;
                let elephant_flow = graph.get(elephant_valve).unwrap().flow;

                if *my_valve == p.me && *elephant_valve == p.elephant {
                    // Both spend the minute to open their respective valves.
                    if !new_state.opened.contains(my_valve) && my_flow > 0 {
                        new_state.opened.insert(my_valve);
                        new_release += my_flow * (26 - minute);
                    }
                    if !new_state.opened.contains(elephant_valve) && elephant_flow > 0 {
                        new_state.opened.insert(elephant_valve);
                        new_release += elephant_flow * (26 - minute);
                    }
                } else if *my_valve == p.me {
                    // I spend the minute opening my valve while the elephant moves.
                    if !new_state.opened.contains(my_valve) && my_flow > 0 {
                        new_state.opened.insert(my_valve);
                        new_release += my_flow * (26 - minute);
                    }
                } else if *elephant_valve == p.elephant {
                    // The elephant spends the minute opening its valve while I move.
                    if !new_state.opened.contains(elephant_valve) && elephant_flow > 0 {
                        new_state.opened.insert(elephant_valve);
                        new_release += elephant_flow * (26 - minute);
                    }
                }

                let new_progress = ProgressTwo::new(my_valve, elephant_valve, new_release);
                // Make sure not to insert needless equivalent states, i.e. switched positions with same total release.
                if !new_states.contains_key(&new_progress)
                    && !new_states.contains_key(&ProgressTwo::new(
                        elephant_valve,
                        my_valve,
                        new_release,
                    ))
                {
                    new_states.insert(new_progress, new_state);
                }
            }
        }
        states = new_states;
    }

    states.keys().map(|p| p.release).max().unwrap()
}

#[derive(Debug)]
struct Valve<'a> {
    flow: usize,
    neighbors: Vec<&'a str>,
}

impl Valve<'_> {
    fn new(flow: usize, neighbors: Vec<&str>) -> Valve {
        Valve { flow, neighbors }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Progress<'a> {
    valve: &'a str, // valve where I am
    release: usize, // total release available
}

impl Progress<'_> {
    fn new(valve: &str, release: usize) -> Progress {
        Progress { valve, release }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct ProgressTwo<'a> {
    me: &'a str,       // valve where I am
    elephant: &'a str, // valve where the elephant is
    release: usize,    // total release available
}

impl ProgressTwo<'_> {
    fn new<'a>(me: &'a str, elephant: &'a str, release: usize) -> ProgressTwo<'a> {
        ProgressTwo {
            me,
            elephant,
            release,
        }
    }
}

#[derive(Debug)]
struct State<'a> {
    opened: HashSet<&'a str>, // valves opened so far
    minute: usize,            // current minute
}

impl State<'_> {
    fn new(opened: HashSet<&str>, minute: usize) -> State {
        State { opened, minute }
    }
}

fn build_input_graph(input: &str) -> HashMap<&str, Valve> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let parts = line.split_once("; ").unwrap();
        let name = parts
            .0
            .strip_prefix("Valve ")
            .unwrap()
            .split_once(' ')
            .unwrap()
            .0;
        let flow = parts
            .0
            .split_once("rate=")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let neighbors = parts
            .1
            .split_once("to ")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
            .1
            .split(", ")
            .collect::<Vec<_>>();
        graph.insert(name, Valve::new(flow, neighbors));
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day16-sample.txt")), 1651);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day16-sample.txt")), 1707);
    }
}
