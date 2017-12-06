use std::env;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
            _ => eprintln!("{} is not implemented", day)
        }
    } else {
        eprintln!("Please specify the dayN to be run");
    }
}
