use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let day: &str = args[1].as_str();
        match day {
            "day1" => day1::run(),
            "day2" => day2::run(),
            _ => eprintln!("{} is not implemented", day)
        }
    } else {
        eprintln!("Please specify the dayN to be run");
    }
}
