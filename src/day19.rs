use std::collections::HashMap;

pub fn solve_1(input: &str) -> i32 {
    let mut cache = HashMap::new();
    build_input_blueprints(input, usize::MAX)
        .iter()
        .enumerate()
        .map(|(idx, blueprint)| {
            i32::try_from(idx + 1).unwrap()
                * solver(blueprint, &State::new(0, 0, 0, 1, 0, 0), 24, &mut cache)
        })
        .sum()
}

pub fn solve_2(input: &str) -> i32 {
    let mut cache = HashMap::new();
    build_input_blueprints(input, 3)
        .iter()
        .map(|blueprint| solver(blueprint, &State::new(0, 0, 0, 1, 0, 0), 32, &mut cache))
        .product()
}

fn build_input_blueprints(input: &str, count: usize) -> Vec<Blueprint> {
    input
        .lines()
        .take(count)
        .map(|line| {
            let sentences = line
                .split_once(": ")
                .unwrap()
                .1
                .split(". ")
                .take(4)
                .collect::<Vec<_>>();

            let ore2ore = sentences[0][21..]
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let ore2cly = sentences[1][22..]
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let ore2obs = sentences[2][26..]
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let cly2obs = sentences[2]
                .split_once(" and ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let ore2geo = sentences[3][23..]
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();
            let obs2geo = sentences[3]
                .split_once(" and ")
                .unwrap()
                .1
                .split_once(' ')
                .unwrap()
                .0
                .parse()
                .unwrap();

            Blueprint::new(ore2ore, ore2cly, ore2obs, cly2obs, ore2geo, obs2geo)
        })
        .collect()
}

fn solver(
    blueprint: &Blueprint,
    state: &State,
    round: i32,
    cache: &mut HashMap<(Blueprint, State, i32), i32>,
) -> i32 {
    if round == 0 {
        return 0;
    }
    // Cache results because the recursion in part 2 is too deep.
    if let Some(value) = cache.get(&(*blueprint, *state, round)) {
        return *value;
    }

    // Build robots in order of priority: geode -> obsidian -> clay -> ore.

    // Build a new geode robot.
    if state.ore_count >= blueprint.ore2geo && state.obs_count >= blueprint.obs2geo {
        return (round - 1)
            + solver(
                blueprint,
                &State::new(
                    state.ore_count + state.ore_robots - blueprint.ore2geo,
                    state.cly_count + state.cly_robots,
                    state.obs_count + state.obs_robots - blueprint.obs2geo,
                    state.ore_robots,
                    state.cly_robots,
                    state.obs_robots,
                ),
                round - 1,
                cache,
            );
    }

    let mut result = 0;

    // Build a new obsidian robot.
    if state.ore_count >= blueprint.ore2obs && state.cly_count >= blueprint.cly2obs {
        result = result.max(solver(
            blueprint,
            &State::new(
                state.ore_count + state.ore_robots - blueprint.ore2obs,
                state.cly_count + state.cly_robots - blueprint.cly2obs,
                state.obs_count + state.obs_robots,
                state.ore_robots,
                state.cly_robots,
                state.obs_robots + 1,
            ),
            round - 1,
            cache,
        ));
    }

    // Build a new clay robot.
    if state.ore_count >= blueprint.ore2cly {
        result = result.max(solver(
            blueprint,
            &State::new(
                state.ore_count + state.ore_robots - blueprint.ore2cly,
                state.cly_count + state.cly_robots,
                state.obs_count + state.obs_robots,
                state.ore_robots,
                state.cly_robots + 1,
                state.obs_robots,
            ),
            round - 1,
            cache,
        ));
    }

    // Build a new ore robot.
    if state.ore_count >= blueprint.ore2ore {
        result = result.max(solver(
            blueprint,
            &State::new(
                state.ore_count + state.ore_robots - blueprint.ore2ore,
                state.cly_count + state.cly_robots,
                state.obs_count + state.obs_robots,
                state.ore_robots + 1,
                state.cly_robots,
                state.obs_robots,
            ),
            round - 1,
            cache,
        ));
    } else {
        // Build nothing.
        result = result.max(solver(
            blueprint,
            &State::new(
                state.ore_count + state.ore_robots,
                state.cly_count + state.cly_robots,
                state.obs_count + state.obs_robots,
                state.ore_robots,
                state.cly_robots,
                state.obs_robots,
            ),
            round - 1,
            cache,
        ));
    }

    cache.insert((*blueprint, *state, round), result);

    result
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct State {
    ore_count: i32,
    cly_count: i32,
    obs_count: i32,
    ore_robots: i32,
    cly_robots: i32,
    obs_robots: i32,
}

impl State {
    fn new(
        ore_count: i32,
        cly_count: i32,
        obs_count: i32,
        ore_robots: i32,
        cly_robots: i32,
        obs_robots: i32,
    ) -> State {
        State {
            ore_count,
            cly_count,
            obs_count,
            ore_robots,
            cly_robots,
            obs_robots,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Blueprint {
    ore2ore: i32,
    ore2cly: i32,
    ore2obs: i32,
    cly2obs: i32,
    ore2geo: i32,
    obs2geo: i32,
}

impl Blueprint {
    fn new(
        ore2ore: i32,
        ore2cly: i32,
        ore2obs: i32,
        cly2obs: i32,
        ore2geo: i32,
        obs2geo: i32,
    ) -> Blueprint {
        Blueprint {
            ore2ore,
            ore2cly,
            ore2obs,
            cly2obs,
            ore2geo,
            obs2geo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1(include_str!("../input/day19-sample.txt")), 33);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2(include_str!("../input/day19-sample.txt")), 3472);
    }
}
