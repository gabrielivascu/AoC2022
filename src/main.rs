use std::{env, error::Error};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        1 => {
            let input_01 = include_str!("../input/day01.txt");
            println!(
                "01: {} {}",
                day01::solve_1(input_01),
                day01::solve_2(input_01)
            );
            let input_02 = include_str!("../input/day02.txt");
            println!(
                "02: {} {}",
                day02::solve_1(input_02),
                day02::solve_2(input_02)
            );
            let input_03 = include_str!("../input/day03.txt");
            println!(
                "03: {} {}",
                day03::solve_1(input_03),
                day03::solve_2(input_03)
            );
            let input_04 = include_str!("../input/day04.txt");
            println!(
                "04: {} {}",
                day04::solve_1(input_04),
                day04::solve_2(input_04)
            );
            let input_05 = include_str!("../input/day05.txt");
            println!(
                "05: {} {}",
                day05::solve_1(input_05),
                day05::solve_2(input_05)
            );
            let input_06 = include_str!("../input/day06.txt");
            println!(
                "06: {} {}",
                day06::solve_1(input_06),
                day06::solve_2(input_06)
            );
            let input_07 = include_str!("../input/day07.txt");
            println!(
                "07: {} {}",
                day07::solve_1(input_07),
                day07::solve_2(input_07)
            );
            let input_08 = include_str!("../input/day08.txt");
            println!(
                "08: {} {}",
                day08::solve_1(input_08),
                day08::solve_2(input_08)
            );
            let input_09 = include_str!("../input/day09.txt");
            println!(
                "09: {} {}",
                day09::solve_1(input_09),
                day09::solve_2(input_09)
            );
            let input_10 = include_str!("../input/day10.txt");
            println!(
                "10: {}\n{}",
                day10::solve_1(input_10),
                day10::solve_2(input_10)
            );
            let input_11 = include_str!("../input/day11.txt");
            println!(
                "11: {} {}",
                day11::solve_1(input_11),
                day11::solve_2(input_11)
            );
            let input_12 = include_str!("../input/day12.txt");
            println!(
                "12: {} {}",
                day12::solve_1(input_12),
                day12::solve_2(input_12)
            );
            let input_13 = include_str!("../input/day13.txt");
            println!(
                "13: {} {}",
                day13::solve_1(input_13),
                day13::solve_2(input_13)
            );
            let input_14 = include_str!("../input/day14.txt");
            println!(
                "14: {} {}",
                day14::solve_1(input_14),
                day14::solve_2(input_14)
            );
            let input_15 = include_str!("../input/day15.txt");
            println!(
                "15: {} {}",
                day15::solve_1(input_15),
                day15::solve_2(input_15)
            );
            let input_16 = include_str!("../input/day16.txt");
            println!(
                "16: {} {}",
                day16::solve_1(input_16),
                day16::solve_2(input_16)
            );
            let input_17 = include_str!("../input/day17.txt");
            println!(
                "17: {} {}",
                day17::solve_1(input_17),
                day17::solve_2(input_17)
            );
            let input_18 = include_str!("../input/day18.txt");
            println!(
                "18: {} {}",
                day18::solve_1(input_18),
                day18::solve_2(input_18)
            );
            let input_19 = include_str!("../input/day19.txt");
            println!(
                "19: {} {}",
                day19::solve_1(input_19),
                day19::solve_2(input_19)
            );
            let input_20 = include_str!("../input/day20.txt");
            println!(
                "20: {} {}",
                day20::solve_1(input_20),
                day20::solve_2(input_20)
            );
            let input_21 = include_str!("../input/day21.txt");
            println!(
                "21: {} {}",
                day21::solve_1(input_21),
                day21::solve_2(input_21)
            );
            let input_22 = include_str!("../input/day22.txt");
            println!(
                "22: {} {}",
                day22::solve_1(input_22),
                day22::solve_2(input_22)
            );
            let input_23 = include_str!("../input/day23.txt");
            println!(
                "23: {} {}",
                day23::solve_1(input_23),
                day23::solve_2(input_23)
            );
            let input_24 = include_str!("../input/day24.txt");
            println!(
                "24: {} {}",
                day24::solve_1(input_24),
                day24::solve_2(input_24)
            );
            let input_25 = include_str!("../input/day25.txt");
            println!(
                "25: {} {}",
                day25::solve_1(input_25),
                day25::solve_2(input_25)
            );
        }
        _ => match args[1].as_str() {
            "1" => {
                let input = include_str!("../input/day01.txt");
                println!("{} {}", day01::solve_1(input), day01::solve_2(input));
            }
            "2" => {
                let input = include_str!("../input/day02.txt");
                println!("{} {}", day02::solve_1(input), day02::solve_2(input));
            }
            "3" => {
                let input = include_str!("../input/day03.txt");
                println!("{} {}", day03::solve_1(input), day03::solve_2(input));
            }
            "4" => {
                let input = include_str!("../input/day04.txt");
                println!("{} {}", day04::solve_1(input), day04::solve_2(input));
            }
            "5" => {
                let input = include_str!("../input/day05.txt");
                println!("{} {}", day05::solve_1(input), day05::solve_2(input));
            }
            "6" => {
                let input = include_str!("../input/day06.txt");
                println!("{} {}", day06::solve_1(input), day06::solve_2(input));
            }
            "7" => {
                let input = include_str!("../input/day07.txt");
                println!("{} {}", day07::solve_1(input), day07::solve_2(input));
            }
            "8" => {
                let input = include_str!("../input/day08.txt");
                println!("{} {}", day08::solve_1(input), day08::solve_2(input));
            }
            "9" => {
                let input = include_str!("../input/day09.txt");
                println!("{} {}", day09::solve_1(input), day09::solve_2(input));
            }
            "10" => {
                let input = include_str!("../input/day10.txt");
                println!("{}\n{}", day10::solve_1(input), day10::solve_2(input));
            }
            "11" => {
                let input = include_str!("../input/day11.txt");
                println!("{} {}", day11::solve_1(input), day11::solve_2(input));
            }
            "12" => {
                let input = include_str!("../input/day12.txt");
                println!("{} {}", day12::solve_1(input), day12::solve_2(input));
            }
            "13" => {
                let input = include_str!("../input/day13.txt");
                println!("{} {}", day13::solve_1(input), day13::solve_2(input));
            }
            "14" => {
                let input = include_str!("../input/day14.txt");
                println!("{} {}", day14::solve_1(input), day14::solve_2(input));
            }
            "15" => {
                let input = include_str!("../input/day15.txt");
                println!("{} {}", day15::solve_1(input), day15::solve_2(input));
            }
            "16" => {
                let input = include_str!("../input/day16.txt");
                println!("{} {}", day16::solve_1(input), day16::solve_2(input));
            }
            "17" => {
                let input = include_str!("../input/day17.txt");
                println!("{} {}", day17::solve_1(input), day17::solve_2(input));
            }
            "18" => {
                let input = include_str!("../input/day18.txt");
                println!("{} {}", day18::solve_1(input), day18::solve_2(input));
            }
            "19" => {
                let input = include_str!("../input/day19.txt");
                println!("{} {}", day19::solve_1(input), day19::solve_2(input));
            }
            "20" => {
                let input = include_str!("../input/day20.txt");
                println!("{} {}", day20::solve_1(input), day20::solve_2(input));
            }
            "21" => {
                let input = include_str!("../input/day21.txt");
                println!("{} {}", day21::solve_1(input), day21::solve_2(input));
            }
            "22" => {
                let input = include_str!("../input/day22.txt");
                println!("{} {}", day22::solve_1(input), day22::solve_2(input));
            }
            "23" => {
                let input = include_str!("../input/day23.txt");
                println!("{} {}", day23::solve_1(input), day23::solve_2(input));
            }
            "24" => {
                let input = include_str!("../input/day24.txt");
                println!("{} {}", day24::solve_1(input), day24::solve_2(input));
            }
            "25" => {
                let input = include_str!("../input/day25.txt");
                println!("{} {}", day25::solve_1(input), day25::solve_2(input));
            }
            _ => return Err(format!("Invalid day: '{}'", args[1]).into()),
        },
    }
    Ok(())
}
