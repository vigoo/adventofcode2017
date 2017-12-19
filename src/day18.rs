
#[derive(Debug)]
enum Error {
    CouldNotParseValue(String),
    CouldNotParseInstruction(String),
    CouldNotParseRegister(String),
    InvalidArgumentCount(String)
}

type Register = char;

fn parse_register(s: &str) -> Result<Register, Error> {
    if s.len() == 1 && s.chars().next().unwrap().is_alphabetic() {
        Ok(s.chars().next().unwrap())
    } else {
        Err(Error::CouldNotParseRegister(String::from(s)))
    }
}

#[derive(Clone)]
enum Value {
    Number(i64),
    RegisterValue(Register)
}

impl Value {
    pub fn parse(s: &str) -> Result<Self, Error> {
        if s.len() == 1 && s.chars().next().unwrap().is_alphabetic() {
            Ok(Value::RegisterValue(s.chars().next().unwrap()))
        } else {
            s.parse::<i64>()
                .map(|value| Value::Number(value))
                .map_err(|_| Error::CouldNotParseValue(String::from(s)))
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value)
}

impl Instruction {
    pub fn parse(line: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        match parts[0] {
            "snd" =>
                if parts.len() == 2 {
                    Value::parse(parts[1]).map(|value| Instruction::Snd(value))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "set" =>
                if parts.len() == 3 {
                    parse_register(parts[1]).and_then(|register|
                        Value::parse(parts[2]).map(|value|
                            Instruction::Set(register, value)))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "add" =>
                if parts.len() == 3 {
                    parse_register(parts[1]).and_then(|register|
                        Value::parse(parts[2]).map(|value|
                            Instruction::Add(register, value)))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "mul" =>
                if parts.len() == 3 {
                    parse_register(parts[1]).and_then(|register|
                        Value::parse(parts[2]).map(|value|
                            Instruction::Mul(register, value)))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "mod" =>
                if parts.len() == 3 {
                    parse_register(parts[1]).and_then(|register|
                        Value::parse(parts[2]).map(|value|
                            Instruction::Mod(register, value)))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "rcv" =>
                if parts.len() == 2 {
                    parse_register(parts[1]).map(|register|
                        Instruction::Rcv(register))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            "jgz" =>
                if parts.len() == 3 {
                    Value::parse(parts[1]).and_then(|condition|
                        Value::parse(parts[2]).map(|offset|
                            Instruction::Jgz(condition, offset)))
                } else {
                    Err(Error::InvalidArgumentCount(String::from(line)))
                },
            _ => Err(Error::CouldNotParseInstruction(String::from(line)))
        }
    }
}

mod part1 {
    use super::{Instruction, Value};
    use common;
    use std::collections::HashMap;

    struct Machine {
        instructions: Vec<Instruction>,
        registers: HashMap<char, i64>,
        position: usize,
        sound: Option<i64>
    }

    impl Machine {
        pub fn init(instructions: Vec<Instruction>) -> Machine {
            Machine {
                instructions,
                registers: HashMap::new(),
                position: 0,
                sound: None
            }
        }

        pub fn step(&mut self) -> bool {
            let mut quit = false;
            match self.instructions[self.position] {
                Instruction::Snd(ref frequency) => {
                    self.sound = Some(self.evaluate(frequency));
                    self.position = self.position + 1;
                },
                Instruction::Set(ref register, ref value) => {
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Add(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value + evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Mul(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value * evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Mod(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value % evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Rcv(ref register) => {
                    if self.evaluate(&Value::RegisterValue(*register)) != 0 {
                        println!("Recovered frequency: {:?}", self.sound);
                        quit = true;
                    }
                    self.position = self.position + 1;
                },
                Instruction::Jgz(ref condition, ref offset) => {
                    if self.evaluate(condition) != 0 {
                        self.position = ((self.position as i64) + self.evaluate(offset)) as usize;
                    } else {
                        self.position = self.position + 1;
                    }
                },
            }
            !quit && self.position != self.instructions.len()
        }

        pub fn run(&mut self) {
            while self.step() {}
        }

        fn evaluate(&self, value: &Value) -> i64 {
            match value {
                &Value::Number(num) => num,
                &Value::RegisterValue(ref register) => *self.registers.get(register).unwrap_or(&0)
            }
        }
    }

    pub fn run() {
        let input = common::read_data("day18.txt");
        let instructions: Vec<Instruction> = input.split("\n").map(Instruction::parse).map(|r| r.unwrap()).collect();
        let mut machine = Machine::init(instructions);
        machine.run();
    }
}

mod part2 {
    use super::{Instruction, Value};
    use common;
    use std::collections::HashMap;
    use std::sync::mpsc::{Sender, Receiver};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    struct Machine {
        instructions: Vec<Instruction>,
        registers: HashMap<char, i64>,
        position: usize,
        sender: Sender<i64>,
        receiver: Receiver<i64>,
        send_counter: i64,
        program_id: i64
    }

    impl Machine {
        pub fn init(instructions: Vec<Instruction>, program_id: i64, sender: Sender<i64>, receiver: Receiver<i64>) -> Machine {
            let mut registers = HashMap::new();
            registers.insert('p', program_id);
            Machine {
                instructions,
                registers,
                position: 0,
                sender,
                receiver,
                send_counter: 0,
                program_id
            }
        }

        pub fn program_id(&self) -> i64 {
            self.program_id
        }

        pub fn step(&mut self) -> bool {
            let mut quit = false;
            match self.instructions[self.position] {
                Instruction::Snd(ref value) => {
                    let evaluated_value = self.evaluate(value);
                    self.sender.send(evaluated_value).ok();
                    self.send_counter = self.send_counter + 1;
                    self.position = self.position + 1;
                },
                Instruction::Set(ref register, ref value) => {
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Add(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value + evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Mul(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value * evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Mod(ref register, ref value) => {
                    let old_value = *self.registers.get(register).unwrap_or(&0);
                    let evaluated_value = self.evaluate(value);
                    self.registers.insert(*register, old_value % evaluated_value);
                    self.position = self.position + 1;
                },
                Instruction::Rcv(ref register) => {
                    match self.receiver.recv_timeout(Duration::from_secs(10)) {
                        Ok(received_value) => {
                            self.registers.insert(*register, received_value);
                            self.position = self.position + 1;
                        },
                        Err(_) => {
                            println!("Program {} failed to receive, quitting", self.program_id());
                            quit = true;
                        }
                    }
                },
                Instruction::Jgz(ref condition, ref offset) => {
                    if self.evaluate(condition) > 0 {
                        self.position = ((self.position as i64) + self.evaluate(offset)) as usize;
                    } else {
                        self.position = self.position + 1;
                    }
                },
            }
            !quit && self.position != self.instructions.len()
        }

        pub fn run(&mut self) {
            while self.step() {}
            println!("Program {} sent data {} times", self.program_id(), self.send_counter);
        }

        fn evaluate(&self, value: &Value) -> i64 {
            match value {
                &Value::Number(num) => num,
                &Value::RegisterValue(ref register) => *self.registers.get(register).unwrap_or(&0)
            }
        }
    }

    pub fn run() {
        let input = common::read_data("day18.txt");
        let instructions: Vec<Instruction> = input.split("\n").map(Instruction::parse).map(|r| r.unwrap()).collect();

        let (send12, recv12) = mpsc::channel();
        let (send21, recv21) = mpsc::channel();

        let thread1 = {
            let instructions1 = instructions.clone();
            thread::spawn(move || {
                let mut machine1 = Machine::init(instructions1, 0, send12, recv21);
                machine1.run();
            })
        };
        let thread2 = {
            let instructions2 = instructions.clone();
            thread::spawn(move || {
                let mut machine2 = Machine::init(instructions2, 1, send21, recv12);
                machine2.run();
            })
        };

        thread1.join().ok();
        thread2.join().ok();
    }
}

pub fn run() {
    part1::run();
    part2::run();
}