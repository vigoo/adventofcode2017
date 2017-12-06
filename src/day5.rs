use common;

struct Machine {
    jumps: Vec<i32>,
    position: i32,
    modifier: fn(i32) -> i32
}

fn modifier1(_offset: i32) -> i32 {
    return 1;
}

fn modifier2(offset: i32) -> i32 {
    if offset >= 3 {
        return -1;
    } else {
        return 1;
    }
}

impl Machine {
    fn new(initial_offsets: Vec<i32>, modifier: fn(i32) -> i32) -> Machine {
        return Machine {
            jumps: initial_offsets,
            position: 0,
            modifier
        };
    }

    fn step(&mut self) -> bool {
        if self.position >= 0 && self.position < (self.jumps.len() as i32) {
            let position: usize = self.position as usize;
            let offset: i32 = self.jumps[position];
            self.jumps[position] += (self.modifier)(offset);
            self.position += offset;
            return true;
        } else {
            return false;
        }
    }

    fn run(&mut self) -> u32 {
        let mut steps: u32 = 0;
        while self.step() {
            steps += 1;
        }
        return steps;
    }

    #[allow(dead_code)]
    fn dump(&self) {
        println!("position: {}", self.position);
        print!  ("memory  : ");
        for offset in self.jumps.iter() {
            print!("{} ", offset);
        }
        println!();
    }
}

fn example() {
    let mut machine = Machine::new(vec![0, 3, 0, 1, -3], modifier1);
    println!("Day 5 example result 1: {}", machine.run());
}

fn read_data() -> Vec<i32> {
    let contents = common::read_data("day5.txt");
    let lines: Vec<&str> = contents.split('\n').collect();
    return lines.iter().map(|s| s.parse::<i32>().unwrap()).collect();
}

fn part1() {
    let initial_offsets = read_data();
    let mut machine = Machine::new(initial_offsets, modifier1);

    println!("Day 5 result 1: {}", machine.run());
}

fn part2() {
    let initial_offsets = read_data();
    let mut machine = Machine::new(initial_offsets, modifier2);

    println!("Day 5 result 2: {}", machine.run());
}

pub fn run() {
    example();
    part1();
    part2();
}