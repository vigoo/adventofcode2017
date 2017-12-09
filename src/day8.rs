extern crate regex;

use common;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;

#[derive(Debug)]
enum ParserError {
    RegexMismatch(String),
    InvalidInstruction,
    InvalidCondition
}

#[derive(Debug, Clone, Eq, Hash)]
struct Register {
    name: String
}

impl Register {
    pub fn from_string(s: &str) -> Register {
        return Register { name: String::from(s) };
    }
}

impl PartialEq for Register {
    fn eq(&self, other: &Register) -> bool {
        return self.name == other.name;
    }
}


#[derive(Debug, Clone)]
enum Condition {
    Greater(Register, i32),
    GreaterOrEq(Register, i32),
    Less(Register, i32),
    LessOrEq(Register, i32),
    Equal(Register, i32),
    NotEqual(Register, i32)
}

impl Condition {
    pub fn from_string(s: &str, register: Register, parameter: i32) -> Result<Condition, ParserError> {
        return match s {
            ">" => Ok(Condition::Greater(register, parameter)),
            ">=" => Ok(Condition::GreaterOrEq(register, parameter)),
            "<" => Ok(Condition::Less(register, parameter)),
            "<=" => Ok(Condition::LessOrEq(register, parameter)),
            "==" => Ok(Condition::Equal(register, parameter)),
            "!=" => Ok(Condition::NotEqual(register, parameter)),
            _ => Err(ParserError::InvalidCondition)
        };
    }

    pub fn all_used_registers(&self) -> HashSet<&Register> {
        match self {
            &Condition::Greater(ref reg, _) => common::singleton(reg),
            &Condition::GreaterOrEq(ref reg, _) => common::singleton(reg),
            &Condition::Less(ref reg, _) => common::singleton(reg),
            &Condition::LessOrEq(ref reg, _) => common::singleton(reg),
            &Condition::Equal(ref reg, _) => common::singleton(reg),
            &Condition::NotEqual(ref reg, _) => common::singleton(reg),
        }
    }

    pub fn is_true(&self, machine: &Machine) -> bool {
        match self {
            &Condition::Greater(ref reg, parameter) => machine.get(reg) > parameter,
            &Condition::GreaterOrEq(ref reg, parameter) => machine.get(reg) >= parameter,
            &Condition::Less(ref reg, parameter) => machine.get(reg) < parameter,
            &Condition::LessOrEq(ref reg, parameter) => machine.get(reg) <= parameter,
            &Condition::Equal(ref reg, parameter) => machine.get(reg) == parameter,
            &Condition::NotEqual(ref reg, parameter) => machine.get(reg) != parameter,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Increase(Register, i32, Condition),
    Decrease(Register, i32, Condition)
}

impl Instruction {
    pub fn from_string(s: &str, register: Register, parameter: i32, condition: Condition) -> Result<Instruction, ParserError> {
        return match s {
            "inc" => Ok(Instruction::Increase(register, parameter, condition)),
            "dec" => Ok(Instruction::Decrease(register, parameter, condition)),
            _ => Err(ParserError::InvalidInstruction)
        }
    }
}

impl Instruction {
    pub fn from_line(line: &str) -> Result<Instruction, ParserError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"([a-z]+) ([a-z]+) (-?[0-9]+) if ([a-z]+) ([<>=!]+) (-?[0-9]+)"#).unwrap();
        }

        let captures = RE.captures(line).ok_or(ParserError::RegexMismatch(String::from(line)))?;

        let register_name = captures.get(1).unwrap().as_str();
        let instruction_str = captures.get(2).unwrap().as_str();
        let parameter = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();

        let condition_register_name = captures.get(4).unwrap().as_str();
        let condition_str = captures.get(5).unwrap().as_str();
        let condition_parameter = captures.get(6).unwrap().as_str().parse::<i32>().unwrap();

        let condition = Condition::from_string(condition_str, Register::from_string(condition_register_name), condition_parameter)?;
        return Instruction::from_string(instruction_str, Register::from_string(register_name), parameter, condition);
    }

    pub fn all_used_registers(&self) -> HashSet<&Register> {
        match self {
            &Instruction::Increase(ref reg, _, ref condition) => common::singleton(reg).union(&condition.all_used_registers()).cloned().collect::<HashSet<&Register>>(),
            &Instruction::Decrease(ref reg, _, ref condition) => common::singleton(reg).union(&condition.all_used_registers()).cloned().collect::<HashSet<&Register>>(),
        }
    }

    pub fn execute(&self, machine: &mut Machine) {
        match self {
            &Instruction::Increase(ref reg, parameter, ref condition) =>
                if condition.is_true(machine) {
                    let input = machine.get(reg);
                    machine.set(reg, input + parameter);
                },
            &Instruction::Decrease(ref reg, parameter, ref condition) =>
                if condition.is_true(machine) {
                    let input = machine.get(reg);
                    machine.set(reg, input - parameter);
                },
        }
    }
}

struct Machine {
    instructions: Vec<Instruction>,
    registers: HashMap<Register, i32>,
    ip: usize,
    peak: i32
}

impl Machine {
    pub fn initialize(instructions: Vec<Instruction>) -> Machine {
        let ip: usize = 0;
        let mut registers: HashMap<Register, i32> = HashMap::new();

        for instruction in instructions.iter() {
            for register in instruction.all_used_registers().iter() {
                registers.insert((*register).clone(), 0);
            }
        }

        return Machine {
            instructions,
            registers,
            ip,
            peak: 0
        };
    }

    pub fn run(&mut self) {
        while self.ip < self.instructions.len() {
            self.step()
        }
    }

    fn step(&mut self) {
        let instruction = self.instructions[self.ip].clone();
        instruction.execute(self);
        self.ip = self.ip + 1
    }

    fn get(&self, register: &Register) -> i32 {
        return self.registers.get(register).unwrap().clone();
    }

    fn set(&mut self, register: &Register, value: i32) {
        self.registers.insert(register.clone(), value);
        self.peak = cmp::max(self.peak, value);
    }
}

pub fn run() {
    let input = common::read_data("day8.txt");
    let instructions: Vec<Instruction> = input.split('\n').map(|line| Instruction::from_line(line).unwrap()).collect();
    let mut machine = Machine::initialize(instructions);
    machine.run();

    let max_register_value = machine.registers.values().max().unwrap();
    println!("Day 8 result 1: {}", max_register_value);
    println!("Day 8 result 2: {}", machine.peak);
}