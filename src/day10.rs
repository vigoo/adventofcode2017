extern crate hex;
use self::hex::ToHex;

struct State {
    list: Vec<u8>,
    position: usize,
    skip_size: usize,
}

fn reverse_section<T : Clone>(list: &mut Vec<T>, start_index: usize, end_index: usize) {
    if end_index != start_index {
        if end_index > start_index {
            list[start_index..end_index].reverse();
        } else {
            let mut items: Vec<T> = list[start_index..].iter().cloned().collect();
            let mut second_part: Vec<T> = list[..end_index].iter().cloned().collect();
            items.append(&mut second_part);
            items.reverse();

            for (i, v) in items.iter().enumerate() {
                let idx = (start_index + i) % list.len();
                list[idx] = v.clone();
            }
        }
    }
}

impl State {
    pub fn new(elem_count: i32) -> State {
        let list: Vec<u8> = (0..elem_count).map(|v| v as u8).collect();
        return State {
            list,
            position: 0,
            skip_size: 0
        }
    }

    pub fn step(&mut self, length: usize) {
        let elem_count = self.list.len();
        let start_index = self.position;
        let end_index = (start_index + length) % elem_count;
        reverse_section(&mut self.list, start_index, end_index);

        self.position = (self.position + length + self.skip_size) % elem_count;
        self.skip_size = self.skip_size + 1;
    }

    pub fn run(&mut self, lengths: &Vec<usize>) {
        for &length in lengths {
            self.step(length);;
        }
    }

    pub fn dense_hash(&self) -> Vec<u8> {
        let elem_count = self.list.len();
        let block_count = elem_count / 16;
        let mut result: Vec<u8> = Vec::with_capacity(block_count);
        for block_idx in 0..block_count {
            let block_start = block_idx * 16;
            let block_end = block_start + 16;
            let code = self.list[block_start..block_end].iter().fold(0, |acc, &v| acc ^ v);
            result.push(code);
        }

        return result;
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for (i, v) in self.list.iter().enumerate() {
            if i == self.position {
                print!("[{}] ", v);
            } else {
                print!("{} ", v);
            }
        }
        println!(" skip_size: {}", self.skip_size);
    }
}

fn example() {
    let mut state = State::new(5);
    state.run(&vec![3, 4, 1, 5]);
    println!("{:?}", state.list)
}

pub fn knot_hash(s: &str) -> Vec<u8> {
    let mut input: Vec<usize> = s.chars().map(|ch| ch as usize).collect();
    for &i in vec![17, 31, 73, 47, 23].iter() {
        input.push(i as usize);
    }
    let mut state = State::new(256);
    for _ in 0..64 {
        state.run(&input);
    }

    return state.dense_hash();
}

fn part1() {
    let mut state = State::new(256);
    state.run(&vec![14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244]);

    println!("Day 10 result 1: {}", (state.list[0] as i32) * (state.list[1] as i32));
}

fn part2() {
    let dense_hash = knot_hash("14,58,0,116,179,16,1,104,2,254,167,86,255,55,122,244");
    let mut result = String::new();
    dense_hash.write_hex(&mut result).expect("Hex conversion failed");
    println!("Day 10 result 2: {}", result);
}

pub fn run() {
    example();
    part1();
    part2();
}