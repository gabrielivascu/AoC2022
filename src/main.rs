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

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        1 => {
            println!("01: {} {}", day01::solve_1(), day01::solve_2());
            println!("02: {} {}", day02::solve_1(), day02::solve_2());
            println!("03: {} {}", day03::solve_1(), day03::solve_2());
            println!("04: {} {}", day04::solve_1(), day04::solve_2());
            println!("05: {} {}", day05::solve_1(), day05::solve_2());
            println!("06: {} {}", day06::solve_1(), day06::solve_2());
            println!("07: {} {}", day07::solve_1(), day07::solve_2());
            println!("08: {} {}", day08::solve_1(), day08::solve_2());
            println!("09: {} {}", day09::solve_1(), day09::solve_2());
            println!("10: {} {}", day10::solve_1(), day10::solve_2());
            println!("11: {} {}", day11::solve_1(), day11::solve_2());
            println!("12: {} {}", day12::solve_1(), day12::solve_2());
            println!("13: {} {}", day13::solve_1(), day13::solve_2());
            println!("14: {} {}", day14::solve_1(), day14::solve_2());
            println!("15: {} {}", day15::solve_1(), day15::solve_2());
            println!("16: {} {}", day16::solve_1(), day16::solve_2());
            println!("17: {} {}", day17::solve_1(), day17::solve_2());
            println!("18: {} {}", day18::solve_1(), day18::solve_2());
            println!("19: {} {}", day19::solve_1(), day19::solve_2());
            println!("20: {} {}", day20::solve_1(), day20::solve_2());
            println!("21: {} {}", day21::solve_1(), day21::solve_2());
            println!("22: {} {}", day22::solve_1(), day22::solve_2());
            println!("23: {} {}", day23::solve_1(), day23::solve_2());
            println!("24: {} {}", day24::solve_1(), day24::solve_2());
            println!("25: {} {}", day25::solve_1(), day25::solve_2());
        }
        _ => match args[1].as_str() {
            "1" => println!("{} {}", day01::solve_1(), day01::solve_2()),
            "2" => println!("{} {}", day02::solve_1(), day02::solve_2()),
            "3" => println!("{} {}", day03::solve_1(), day03::solve_2()),
            "4" => println!("{} {}", day04::solve_1(), day04::solve_2()),
            "5" => println!("{} {}", day05::solve_1(), day05::solve_2()),
            "6" => println!("{} {}", day06::solve_1(), day06::solve_2()),
            "7" => println!("{} {}", day07::solve_1(), day07::solve_2()),
            "8" => println!("{} {}", day08::solve_1(), day08::solve_2()),
            "9" => println!("{} {}", day09::solve_1(), day09::solve_2()),
            "10" => println!("{} {}", day10::solve_1(), day10::solve_2()),
            "11" => println!("{} {}", day11::solve_1(), day11::solve_2()),
            "12" => println!("{} {}", day12::solve_1(), day12::solve_2()),
            "13" => println!("{} {}", day13::solve_1(), day13::solve_2()),
            "14" => println!("{} {}", day14::solve_1(), day14::solve_2()),
            "15" => println!("{} {}", day15::solve_1(), day15::solve_2()),
            "16" => println!("{} {}", day16::solve_1(), day16::solve_2()),
            "17" => println!("{} {}", day17::solve_1(), day17::solve_2()),
            "18" => println!("{} {}", day18::solve_1(), day18::solve_2()),
            "19" => println!("{} {}", day19::solve_1(), day19::solve_2()),
            "20" => println!("{} {}", day20::solve_1(), day20::solve_2()),
            "21" => println!("{} {}", day21::solve_1(), day21::solve_2()),
            "22" => println!("{} {}", day22::solve_1(), day22::solve_2()),
            "23" => println!("{} {}", day23::solve_1(), day23::solve_2()),
            "24" => println!("{} {}", day24::solve_1(), day24::solve_2()),
            "25" => println!("{} {}", day25::solve_1(), day25::solve_2()),
            _ => return Err(format!("Invalid day: '{}'", args[1]).into()),
        },
    }
    Ok(())
}
