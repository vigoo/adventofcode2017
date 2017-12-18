use common;
use std::collections::HashMap;

#[derive(Debug)]
enum Error {
    CouldNotParseStep(String)
}

type Program = char;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Move {
    pub fn parse(s: &str) -> Result<Move, Error> {
        return match s.chars().next() {
            Some('s') => s[1..].parse::<usize>().map(|x| Move::Spin(x)).map_err(|_| Error::CouldNotParseStep(String::from(s))),
            Some('x') => {
                let parts: Vec<&str> = s[1..].split("/").collect();
                if parts.len() != 2 {
                    return Err(Error::CouldNotParseStep(String::from(s)));
                } else {
                    return parts[0].parse::<usize>()
                        .and_then(|a| {
                            parts[1].parse::<usize>().map(|b| {
                                Move::Exchange(a, b)
                            })
                        })
                        .map_err(|_| Error::CouldNotParseStep(String::from(s)));
                }
            },
            Some('p') => {
                let parts: Vec<&str> = s[1..].split("/").collect();
                if parts.len() != 2 || parts[0].len() != 1 || parts[1].len() != 1 {
                    return Err(Error::CouldNotParseStep(String::from(s)));
                } else {
                    return Ok(Move::Partner(parts[0].chars().next().unwrap(), parts[1].chars().next().unwrap()));
                }
            }
            _ => Err(Error::CouldNotParseStep(String::from(s)))
        }
    }
}

struct State {
    line: Vec<Program>,
    index: HashMap<char, usize>,
    recorded :HashMap<String, (String, usize)>
}

impl State {
    pub fn initial() -> Self {
        let line: Vec<char> = (0..16).map(|n| (n + 'a' as u8) as char).collect();
        let index = HashMap::new();

        let mut result = State { line, index, recorded: HashMap::new() };
        result.reindex();
        result
    }

    fn reindex(&mut self) {
        self.index.clear();
        for (i, &c) in self.line.iter().enumerate() {
            self.index.insert(c, i);
        }
    }

    fn perfom(&mut self, mov: &Move) {
        match mov {
            &Move::Spin(x) => {
                for _ in 0..x {
                    let program = self.line.pop().unwrap();
                    self.line.insert(0, program);
                }
                self.reindex();
            }
            &Move::Exchange(a, b) => {
                let program_a = self.line[a];
                let program_b = self.line[b];
                self.line[a] = program_b;
                self.line[b] = program_a;
                self.index.insert(program_b, a);
                self.index.insert(program_a, b);
            }
            &Move::Partner(a, b) => {
                let a_idx = self.index[&a];
                let b_idx = self.index[&b];

                self.line[a_idx] = b;
                self.line[b_idx] = a;
                self.index.insert(a, b_idx);
                self.index.insert(b, a_idx);
            }
        }
    }

    pub fn run(&mut self, n: usize, moves: &Vec<Move>) -> Option<usize> {
        let from = self.to_string();

        if self.recorded.contains_key(&from) {
            let &(ref to, prev_idx) = self.recorded.get(&from).unwrap();
            self.line = to.chars().collect();
            Some(prev_idx)
        } else {
            self.reindex();
            for mov in moves {
                self.perfom(&mov);
            }

            let to = self.to_string();
            self.recorded.insert(from, (to, n));
            None
        }
    }

    pub fn dump(&self) {
        for program in self.line.iter() {
            print!("{}", program);
        }
        println!();
    }

    pub fn to_string(&self) -> String {
        self.line.iter().collect()
    }
}

pub fn run() {
    let input = common::read_data("day16.txt");
    let moves: Vec<Move> = input.split(",").map(Move::parse).map(|r| r.unwrap()).collect();
    println!("There are {} moves in one cycle", moves.len());

    let mut state = State::initial();

    print!("Initial state: ");
    state.dump();

    state.run(0, &moves);

    print!("State after one cycle: ");
    state.dump();

    const N: usize = 1000000000;
    let mut n: usize = 1;
    while n < (N - 1) {
        let from = state.to_string();
        let cycled = state.run(n, &moves);
        let to = state.to_string();
        println!("{}: {} => {} {:?}", n, from, to, cycled);
        match cycled {
            Some(prev_idx) => {
                let d0 = (n - prev_idx) as usize;
                let d = d0 * ((N - n) / d0);
                if d > 0 {
                    println!("Found cycle of length {} at {}, jumping to {}", d0, n, n + d);
                    n = n + d;
                } else {
                    n = n + 1;
                }
            },
            None =>
                n = n + 1
        }
    }

    print!("Final state: ");
    state.dump();
}