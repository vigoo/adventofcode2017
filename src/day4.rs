use std::collections::HashSet;
use std::env;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::fs::File;

fn is_valid_1(s: &str) -> bool {
    let all_words: Vec<&str> = s.split(' ').collect();
    let unique_words: HashSet<&str> = HashSet::from_iter(s.split(' '));
    return unique_words.len() == all_words.len();
}

fn sorted_chars_of(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().into_iter().collect();
    chars.sort();
    return String::from_iter(chars.iter());
}

fn is_valid_2(s: &str) -> bool {
    let all_words: Vec<&str> = s.split(' ').collect();
    let unique_words: HashSet<String> =
        HashSet::from_iter(all_words.iter().map(|s| sorted_chars_of(*s)));

    return unique_words.len() == all_words.len();
}

pub fn run() {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push("day4.txt");

    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let all_lines: Vec<&str> = contents.split("\n").collect();
    let valid_lines_1: Vec<&&str> = all_lines.iter().filter(|&s|is_valid_1(s)).collect();
    let valid_lines_2: Vec<&&str> = all_lines.iter().filter(|&s|is_valid_2(s)).collect();

    println!("Day 4 answer 1: {}", valid_lines_1.len());
    println!("Day 4 answer 2: {}", valid_lines_2.len());
}
