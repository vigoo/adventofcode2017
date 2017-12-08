extern crate regex;

use common;
use self::regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum ParserError {
    RegexMismatch(String),
    InvalidInstruction,
    InvalidCondition
}

struct Register {
    name: String
}

impl Register {
    pub fn from_string(s: &str) -> Register {
        return Register { name: String::from(s) };
    }
}

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
}

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
}

pub fn run() {
    let input = common::read_data("day8.txt");
    let instructions: Vec<Instruction> = input.split('\n').map(|line| Instruction::from_line(line).unwrap()).collect();
}