use common;

struct LayerSpec {
    idx: usize,
    range: i32
}

impl LayerSpec {
    pub fn parse(line: &str) -> LayerSpec {
        let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        return LayerSpec {
            idx: parts[0].parse::<usize>().unwrap(),
            range: parts[1].parse::<i32>().unwrap()
        }
    }
}

struct LayerState {
    range: i32,
    state: i32,
    direction: i32
}

impl LayerState {
    pub fn empty() -> LayerState {
        return LayerState {
            range: 0,
            state: 0,
            direction: 0
        };
    }

    pub fn from(spec: &LayerSpec) -> LayerState {
        return LayerState {
            range: spec.range,
            state: 0,
            direction: 1
        }
    }
}

struct Simulation {
    layers: Vec<LayerState>,
    position: usize,
    total_severity: i32,
    caught: i32
}

impl Simulation {
    pub fn init(specs: &Vec<LayerSpec>) -> Simulation {
        let layer_count = specs.iter().map(|spec| spec.idx).max().unwrap() + 1;
        let mut layers: Vec<LayerState> = Vec::with_capacity(layer_count);
        for _idx in 0..layer_count {
            layers.push(LayerState::empty());
        }
        for spec in specs {
            layers[spec.idx] = LayerState::from(spec);
        }

        return Simulation {
            layers,
            position: 0,
            total_severity: 0,
            caught: 0
        };
    }

    fn step_layers(&mut self) {
        for layer in self.layers.iter_mut() {
            let next_state = layer.state + layer.direction;
            if (next_state >= layer.range) || (next_state < 0) {
                layer.direction = layer.direction * -1;
                layer.state = layer.state + layer.direction;
            } else {
                layer.state = next_state;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let current_severity = self.current_severity();
        self.total_severity = self.total_severity + current_severity.unwrap_or(0);
        self.caught = self.caught + if current_severity.is_some() { 1 } else { 0 };
        self.step_layers();

        self.position = self.position + 1;
        return self.position != self.layers.len();
    }

    pub fn delay(&mut self, t: i32) {
        for _idx in 0..t {
            self.step_layers();
        }
    }

    pub fn run(&mut self, stop_when_caught: bool, dump: bool) {
        if dump {
            self.dump();
        }
        while self.step() {
            if dump {
                self.dump();
            }
            if stop_when_caught && self.caught > 0 {
                break;
            }
        };
    }

    fn current_severity(&self) -> Option<i32> {
        let current_layer: &LayerState = &self.layers[self.position];
        if current_layer.range > 0 && current_layer.state == 0 {
            return Some((self.position as i32) * current_layer.range);
        } else {
            return None;
        }
    }

    #[allow(dead_code)]
    fn dump(&self) {
        let max_range = self.layers.iter().map(|layer| layer.range).max().unwrap();
        for r in 0..(max_range + 1) {
            for i in 0..self.layers.len() {
                if r == 0 {
                    print!(" {}  ", i);
                } else {
                    let is_current = self.position == i && r == 1;
                    if self.layers[i].range >= r {
                        if self.layers[i].state == (r - 1) {
                            if is_current {
                            print!("(S) ");
                            } else {
                                print!("[S] ");
                            }
                        } else {
                            if is_current {
                                print!("( ) ");
                            } else {
                                print!("[ ] ");
                            }

                        }
                    } else {
                        if is_current {
                            print!("(.) ");
                        } else {
                            print!("... ");
                        }
                    }
                }
            }
            println!();
        }
        println!("=> {}, {}", self.caught, self.total_severity);
        println!();
    }
}

fn part1(name: &str, layer_specs: &Vec<LayerSpec>, dump: bool) {
    let mut simulation = Simulation::init(layer_specs);

    simulation.run(false, dump);
    println!("{} result 1: {}", name, simulation.total_severity);
}

fn try_with_delay(layer_specs: &Vec<LayerSpec>, delay: i32, dump: bool) -> bool {
    println!("Trying with delay {}", delay);
    let mut simulation = Simulation::init(layer_specs);
    simulation.delay(delay);
    simulation.run(true, dump);
    println!();
    return simulation.caught == 0;
}

fn part2(name: &str, layer_specs: &Vec<LayerSpec>, dump: bool) {
    let mut delay = 0;

    while !try_with_delay(layer_specs, delay, dump) {
        delay = delay + 1;
    }

    println!("{} result 2: {}", name, delay);
}

fn example() {
    let layer_specs = vec![
        LayerSpec::parse("0: 3"),
        LayerSpec::parse("1: 2"),
        LayerSpec::parse("4: 4"),
        LayerSpec::parse("6: 4")
    ];

    part1("Example", &layer_specs, true);
    part2("Example", &layer_specs, true);
}

pub fn run() {
    example();

    let input = common::read_data("day13.txt");
    let layer_specs: Vec<LayerSpec> = input.split("\n").map(|line| LayerSpec::parse(line)).collect();
    part1("Day 13", &layer_specs, false);
    part2("Day 13", &layer_specs, false);
}