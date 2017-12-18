use common;

struct Result {
    score: u32,
    garbage_count: i32
}

fn process(input: &String) -> Result {
    let mut level = 0;
    let mut score: u32 = 0;
    let mut ignore_next: bool = false;
    let mut in_garbage: bool = false;
    let mut garbage_count: i32 = 0;

    for ch in input.chars() {
        if !ignore_next {
            match ch {
                '{' if !in_garbage => {
                    level = level + 1;
                },
                '}' if !in_garbage => {
                    score = score + level;
                    level = level - 1;
                },
                '<' if !in_garbage => {
                    in_garbage = true;
                    garbage_count = garbage_count - 1; // leading < does not count
                },
                '>' => {
                    in_garbage = false;
                },
                '!' => {
                    ignore_next = true;
                }
                _ => {}
            }

            if in_garbage && ch != '>' && ch != '!' {
                garbage_count = garbage_count + 1;
            }
        } else {
            ignore_next = false;
        }
    }

    Result {
        score,
        garbage_count
    }
}

fn print_score(input: &str) {
    let result = process(&String::from(input));
    println!("{} score: {}", input, result.score);
}

fn examples() {
    print_score("{}");
    print_score("{{{}}}");
    print_score("{{},{}}");
    print_score("{{{},{},{{}}}}");
    print_score("{<a>,<a>,<a>,<a>}");
    print_score("{{<ab>},{<ab>},{<ab>},{<ab>}}");
    print_score("{{<!!>},{<!!>},{<!!>},{<!!>}}");
    print_score("{{<a!>},{<a!>},{<a!>},{<ab>}}");

    print_garbage_count("<>");
    print_garbage_count("<random characters>");
    print_garbage_count("<<<<>");
    print_garbage_count("<{!>}>");
    print_garbage_count("<!!>");
    print_garbage_count("<!!!>>");
    print_garbage_count("<{o\"i!a,<{i<a>");
}

fn print_garbage_count(input: &str) {
    let result = process(&String::from(input));
    println!("{} garbage count: {}", input, result.garbage_count);
}

pub fn run() {
    let input = common::read_data("day9.txt");
    let result = process(&input);

    examples();
    println!("Day 9 result 1: {}", result.score);
    println!("Day 9 result 2: {}", result.garbage_count);
}