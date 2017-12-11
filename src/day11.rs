use common;
use std::cmp;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
enum Error {
    CouldNotParseStep(String)
}

enum Step {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest
}

impl Step {
    pub fn from_string(s: &str) -> Result<Step, Error> {
        return match s {
            "n" => Ok(Step::North),
            "ne" => Ok(Step::NorthEast),
            "se" => Ok(Step::SouthEast),
            "s" => Ok(Step::South),
            "sw" => Ok(Step::SouthWest),
            "nw" => Ok(Step::NorthWest),
            _ => Err(Error::CouldNotParseStep(String::from(s)))
        };
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash)]
struct Coords {
    x: i32,
    y: i32,
}

impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Coords {
    pub fn apply_step(&mut self, step: &Step) {
        return match step {
            &Step::North => self.apply_d(0, -2),
            &Step::NorthEast => self.apply_d(1, -1),
            &Step::SouthEast=> self.apply_d(1, 1),
            &Step::South => self.apply_d(0, 2),
            &Step::SouthWest => self.apply_d(-1, 1),
            &Step::NorthWest => self.apply_d(-1, -1)
        }
    }

    pub fn step(&self, step: &Step) -> Coords {
        let mut cloned = self.clone();
        cloned.apply_step(step);
        return cloned;
    }

    fn apply_d(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }

    fn dist2(&self, other: &Coords) -> i64 {
        let dx: i64 = (other.x - self.x) as i64;
        let dy: i64 = (other.y - self.y) as i64;
        return dx*dx + dy*dy;
    }
}

#[derive(Debug, Eq)]
struct SearchStep {
    state: Coords,
    history: u32,
    cost: i64
}

impl PartialEq for SearchStep {
    fn eq(&self, other: &SearchStep) -> bool {
        return self.state == other.state && self.history == other.history && self.cost == other.cost;
    }
}

impl Ord for SearchStep {
    fn cmp(&self, other: &SearchStep) -> cmp::Ordering {
        return self.cost.partial_cmp(&other.cost).map(|o| o.reverse()).unwrap();
    }
}

impl PartialOrd for SearchStep {
    fn partial_cmp(&self, other: &SearchStep) -> Option<cmp::Ordering> {
        return Some(self.cost.partial_cmp(&other.cost).map(|o| o.reverse()).unwrap());
    }
}

impl SearchStep {
    pub fn next(&self, step: &Step, goal: &Coords) -> SearchStep {
        let next_state = self.state.step(step);
        return SearchStep {
            state: next_state,
            history: self.history + 1,
            cost: next_state.dist2(goal)
        };
    }
}

fn valid_steps(_coords: &Coords) -> Vec<Step> {
    return vec![Step::North, Step::NorthEast, Step::SouthEast, Step::South, Step::SouthWest, Step::NorthWest];
}

fn find_path(start: &Coords, goal: &Coords) -> u32 {
    let mut search_steps: BinaryHeap<SearchStep> = BinaryHeap::new();
    let mut visited: HashSet<Coords> = HashSet::new();
    let mut result = None;

    search_steps.push(SearchStep {
        state: start.clone(),
        history: 0,
        cost: 0
    });

    while let Some(step) = search_steps.pop() {

        if step.state == *goal {
            result = Some(step);
            search_steps.clear();
        } else {
            for next_step in valid_steps(&step.state) {
                let next_coords = step.state.step(&next_step);
                if !visited.contains(&next_coords) {
                    search_steps.push(step.next(&next_step, goal));
                    visited.insert(next_coords);
                }
            }
        }
    }

    return result.unwrap().history;
}

pub fn run() {
    let input = common::read_data("day11.txt");
    let step_strs: Vec<Step> = input.split(",").map(|s| Step::from_string(s).unwrap()).collect();
    let start = Coords { x: 0, y: 0};
    let (child_location, furhest_steps) =
        step_strs.iter().fold((start, None), |(acc, current_furthest), step| {
            print!(".");
            let next = acc.step(step);
            let path_len = find_path(&start, &next);
            let updated_furthest: Option<u32> =
                match current_furthest {
                    Some(furthest) => Some(cmp::max(path_len, furthest)),
                    None => Some(path_len)
                };

            return (next, updated_furthest);
        });

    println!("Child's final location: {:?}", child_location);

    println!("Day 11 result 1: {}", find_path(&start, &child_location));
    println!("Day 11 result 2: {}", furhest_steps.unwrap());
}