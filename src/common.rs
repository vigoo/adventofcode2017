use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::hash::Hash;

pub fn read_data(name: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push("data");
    path.push(name);

    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    contents
}

pub fn singleton<T: Eq + Hash>(value: T) -> HashSet<T> {

    let mut set = HashSet::new();
    set.insert(value);

    set
}