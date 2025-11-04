use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::Day;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left(usize),
    Right(usize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match *s.as_bytes() {
            [b'R', ..] => Self::Right(s[1..].parse()?),
            [b'L', ..] => Self::Left(s[1..].parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    names: Vec<String>,
    instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let names = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .split(',')
            .map(str::to_string)
            .collect();
        if lines.next().is_none_or(|l| !l.is_empty()) {
            return Err(ParseError::SyntaxError);
        }
        let instructions = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            names,
            instructions,
        })
    }
}

pub struct Day01;

impl Day for Day01 {
    type Input = Input;
    type ParseError = ParseError;
    type Output1 = String;
    type Output2 = String;
    type Output3 = String;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut pos = 0_usize;
        for &instr in &input.instructions {
            match instr {
                Instruction::Left(n) => pos = pos.saturating_sub(n),
                Instruction::Right(n) => pos = (pos + n).min(input.names.len() - 1),
            }
        }
        input.names[pos].clone()
    }

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut pos = 0_usize;
        let len = input.names.len();
        for &instr in &input.instructions {
            match instr {
                Instruction::Left(n) => pos = (pos + len - n % len) % len,
                Instruction::Right(n) => pos = (pos + n) % len,
            }
        }
        input.names[pos].clone()
    }

    fn part_3(input: &Self::Input) -> Self::Output3 {
        let mut names = input.names.clone();
        let len = input.names.len();
        for &instr in &input.instructions {
            match instr {
                Instruction::Left(n) => names.swap(0, (len - n % len) % len),
                Instruction::Right(n) => names.swap(0, n % len),
            }
        }
        names[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        Vyrdax,Drakzyph,Fyrryn,Elarzris\n\
        \n\
        R3,L2,R3,L1\
    ";

    const EXAMPLE2: &str = "\
        Vyrdax,Drakzyph,Fyrryn,Elarzris\n\
        \n\
        R3,L2,R3,L3\
    ";

    #[test]
    fn test_parse() {
        let result: Input = EXAMPLE1.parse().unwrap();
        assert_eq!(result.names, ["Vyrdax", "Drakzyph", "Fyrryn", "Elarzris"]);
        assert_eq!(
            result.instructions,
            [
                Instruction::Right(3),
                Instruction::Left(2),
                Instruction::Right(3),
                Instruction::Left(1),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = EXAMPLE1.parse().unwrap();
        let result = Day01::part_1(&input);
        assert_eq!(result, "Fyrryn");
    }

    #[test]
    fn test_part_2() {
        let input = EXAMPLE1.parse().unwrap();
        let result = Day01::part_2(&input);
        assert_eq!(result, "Elarzris");
    }

    #[test]
    fn test_part_3() {
        let input = EXAMPLE2.parse().unwrap();
        let result = Day01::part_3(&input);
        assert_eq!(result, "Drakzyph");
    }
}
