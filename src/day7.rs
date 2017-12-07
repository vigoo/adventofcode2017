extern crate regex;

use common;
use self::regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weigth: i32,
    parent: Option<String>,
    children: Vec<String>
}

impl Program {
    pub fn from_line(line: &str) -> Program {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"([a-z]+) \((\d+)\)( -> ([a-z, ]+))?"#).unwrap();
        }

        let captures = RE.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let weigth = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let child_list = captures.get(4).map_or("", |m| m.as_str());
        let child_names: Vec<String> = child_list
            .split(',')
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| String::from(s))
            .collect();

        return Program {
            name: String::from(name),
            weigth,
            parent: None,
            children: child_names
        }
    }
}

struct Tree {
    programs: HashMap<String, Program>
}

#[derive(Clone, Debug)]
struct Correction {
    program: String,
    correction: i32
}

fn create_map(programs: &Vec<Program>) -> HashMap<String, Program> {
    let mut result: HashMap<String, Program> = HashMap::new();

    // Setting up dictionary
    for program in programs {
        result.insert(program.name.clone(), program.clone());
    }

    return result;
}

fn find_outlier(weights: &Vec<i32>) -> (i32, i32) {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &weight in weights {
        let count = counts.entry(weight).or_insert(0).clone();
        counts.insert(weight, count + 1);
    }

    let mut pairs: Vec<(&i32, &i32)> = counts.iter().collect();
    pairs.sort_by_key(|&(_, &c)| -c);

    assert_eq!(pairs.len(), 2);

    let (wgood, _) = pairs[0];
    let (wbad, _) = pairs[1];

    return (*wgood, *wbad);
}

impl Tree {
    pub fn from_programs(programs: &Vec<Program>) -> Tree {
        let mut result = create_map(programs);

        // Setting parent references
        for program in programs {
            for child in program.children.iter() {
                match result.get_mut(&*child) {
                    Some(child_program) => child_program.parent = Some(program.name.clone()),
                    None => eprintln!("Could not find {}", child)
                }
            }
        }

        return Tree { programs: result };
    }

    pub fn root(&self) -> Option<&Program> {
        return match self.programs.iter().find(|&(_name, program)| program.parent.is_none()) {
            Some((_name, program)) => Some(program),
            None => None
        }
    }

    pub fn total_weight(&self, program: &Program) -> i32 {
        if program.children.is_empty() {
            return program.weigth;
        } else {
            let child_weights: i32 = program.children.iter().map(|child| self.total_weight(self.programs.get(child).unwrap())).sum();
            return program.weigth + child_weights;
        }
    }

    pub fn find_correction(&self, program: &Program) -> Option<Correction> {
        if program.children.is_empty() {
            return None;
        } else {
            let weights: Vec<i32> = program.children.iter().map(|child| self.total_weight(self.programs.get(child).unwrap())).collect();
            let children_corrections: Vec<Option<Correction>> = program.children.iter().map(|child| self.find_correction(self.programs.get(child).unwrap())).collect();
            let children_balanced = children_corrections.iter().all(|c| c.is_none());
            let weights_good = weights.iter().all(|w| *w == weights[0]);

            if children_balanced && weights_good {
                return None;
            } else {
                if children_balanced && !weights_good {
                    for idx in 0..weights.len() {
                        let child = self.programs.get(&program.children[idx]).unwrap();

                        let (good_weight, wrong_weight) = find_outlier(&weights);

                        if weights[idx] == wrong_weight {
                            let diff = wrong_weight - good_weight;
                            return Some(Correction { program: child.name.clone(), correction: child.weigth - diff });
                        }
                    }

                    return None;

                } else {
                    let cs: Vec<Correction> = children_corrections
                        .iter().filter(|c| c.is_some())
                        .map(|c| c.clone().unwrap().clone()).collect();
                    return cs.first().map(|c| c.clone());
                }
            }
        }
    }
}

pub fn run() {
    let input = common::read_data("day7.txt");
    let programs: Vec<Program> = input.split('\n').map(Program::from_line).collect();
    let tree = Tree::from_programs(&programs);

    let root = tree.root().unwrap();
    println!("Day 7 result 1: {:?}", root);
    println!("Day 7 result 2: {:?}", tree.find_correction(root).map(|c| c.correction));
}