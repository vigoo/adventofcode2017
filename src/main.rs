#[macro_use] extern crate lazy_static;

use std::env;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let day: &str = args[1].as_str();
        match day {
            "day1" => day1::run(),
            "day2" => day2::run(),
            "day3" => day3::run(),
            "day4" => day4::run(),
            "day5" => day5::run(),
            "day6" => day6::run(),
            "day7" => day7::run(),
            "day8" => day8::run(),
            "day9" => day9::run(),
            "day10" => day10::run(),
            "day11" => day11::run(),
            "day12" => day12::run(),
            "day13" => day13::run(),
            "day14" => day14::run(),
            "day15" => day15::run(),
            _ => eprintln!("{} is not implemented", day)
        }
    } else {
        eprintln!("Please specify the dayN to be run");
    }
}
