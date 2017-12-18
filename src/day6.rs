use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash)]
struct State {
    banks: Vec<u32>
}

impl PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        self.banks.eq(&other.banks)
    }
}

impl State {
    fn new(banks: Vec<u32>) -> Self {
        State { banks }
    }

    fn find_max(&self) -> (usize, u32) {
        let (idx, max) = self.banks.iter().enumerate().min_by_key(|&(_, &value)| -(value as i32)).unwrap();
        (idx, max.clone())
    }

    fn redistribute(&mut self) {
        assert!(self.banks.len() > 0);

        let (start, max) = self.find_max();
        let len = self.banks.len();
        let mut remaining = max;
        let mut idx = start;

        self.banks[start] = 0;
        while remaining > 0 {
            idx = (idx + 1) % len;
            remaining = remaining - 1;
            self.banks[idx] = self.banks[idx] + 1;
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for blocks in self.banks.iter() {
            print!("{} ", blocks);
        }
        println!();
    }
}

fn count_redistribution_cycles(initial_state: &State, with_cycle_length: bool) -> (u32, Option<u32>) {
    let mut visited_states: HashSet<State> = HashSet::new();

    let mut state = initial_state.clone();
    let mut count = 0;
    while !visited_states.contains(&state) {
        visited_states.insert(state.clone());
        state.redistribute();
        count = count + 1;
    }

    let cycle_length =
        if with_cycle_length {
            let (l, _) = count_redistribution_cycles(&state, false);
            Some(l)
        } else {
            None
        };
    (count, cycle_length)
}

fn example() {
    let initial_state = State::new(vec![0, 2, 7, 0]);
    println!("Example results: {:?}", count_redistribution_cycles(&initial_state, true));
}

fn puzzle_input() -> State {
    let input = "4	10	4	1	8	4	9	14	5	1	14	15	0	15	3	5";
    return State::new(input.split_whitespace().map(|s| s.trim().parse::<u32>().unwrap()).collect());
}

fn part1() {
    let initial_state = puzzle_input();
    println!("Day 6 results: {:?}", count_redistribution_cycles(&initial_state, true));
}

pub fn run() {
    example();
    part1();
}