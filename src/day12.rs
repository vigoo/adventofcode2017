use common;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Eq, Hash)]
struct Program {
    id: i32
}

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        self.id == other.id
    }
}

fn parse_program(s: &str) -> Program {
    Program { id: s.parse::<i32>().unwrap() }
}

fn parse_line(line: &str) -> (Program, HashSet<Program>) {
    let parts1: Vec<&str> = line.split("<->").collect();
    let targets: HashSet<Program> = parts1[1].split(',').map(|s| s.trim()).map(parse_program).collect();

    (parse_program(parts1[0].trim()), targets)
}

fn get_accessible_programs(connections: &HashMap<Program, HashSet<Program>>, from: Program) -> HashSet<Program> {
    let mut visited: HashSet<Program> = HashSet::new();
    let mut queue: VecDeque<Program> = VecDeque::new();
    queue.push_back(from);

    while let Some(current) = queue.pop_front() {
        for target in connections.get(&current).unwrap().iter() {
            if !visited.contains(&target) {
                queue.push_back(target.clone());
            }
        }
        visited.insert(current);
    }

    visited
}

fn count_accessible_programs(connections: &HashMap<Program, HashSet<Program>>) -> usize {
    get_accessible_programs(connections, Program { id: 0 }).len()
}

fn count_components(connections: &HashMap<Program, HashSet<Program>>) -> usize {
    let mut component_count: usize = 0;
    let mut visited: HashSet<Program> = HashSet::new();

    for program in connections.keys() {
        if !visited.contains(program) {
            let component = get_accessible_programs(connections, *program);
            for p in component {
                visited.insert(p);
            }
            component_count = component_count + 1;
        }
    }

    component_count
}

pub fn run() {
    let input = common::read_data("day12.txt");
    let connections: HashMap<Program, HashSet<Program>> = input.split("\n").map(parse_line).collect();
    println!("Day 12 result 1: {}", count_accessible_programs(&connections));
    println!("Day 12 result 2: {}", count_components(&connections));
}