use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    first: u64,
    pairs: Vec<(u64, u64)>,
    last: u64,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .parse::<u64>()?;
        let mut pairs = Vec::new();
        let mut last = 0;
        if let Some(mut prev) = lines.next() {
            for line in lines {
                if let Some((left, right)) = prev.split_once('|') {
                    pairs.push((left.parse()?, right.parse()?));
                }
                prev = line;
            }
            last = prev.parse()?;
        }
        Ok(Self { first, pairs, last })
    }
}

pub struct Day04;

impl crate::Day for Day04 {
    type Input = Input;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    type Output1 = u64;
    fn part_1(gears: &Self::Input) -> Self::Output1 {
        gears.first * 2025 / gears.last
    }

    type Output2 = u64;
    fn part_2(gears: &Self::Input) -> Self::Output2 {
        (10_000_000_000_000 * gears.last).div_ceil(gears.first)
    }

    type Output3 = u64;
    fn part_3(input: &Self::Input) -> Self::Output3 {
        let mut teeth = 100 * input.first;
        for &(left, right) in &input.pairs {
            teeth = teeth * right / left;
        }
        teeth / input.last
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
        128\n\
        64\n\
        32\n\
        16\n\
        8\
    ";

    const EXAMPLE2: &str = "\
        102\n\
        75\n\
        50\n\
        35\n\
        13\
    ";

    const EXAMPLE3: &str = "\
        5\n\
        5|10\n\
        10|20\n\
        5\
    ";

    const EXAMPLE4: &str = "\
        5\n\
        7|21\n\
        18|36\n\
        27|27\n\
        10|50\n\
        10|50\n\
        11\
    ";

    #[test_case(EXAMPLE1 => 32_400)]
    #[test_case(EXAMPLE2 => 15_888)]
    fn test_part_1(input: &str) -> u64 {
        let gears = Day04::parse(input).unwrap();
        Day04::part_1(&gears)
    }

    #[test_case(EXAMPLE1 => 625_000_000_000)]
    #[test_case(EXAMPLE2 => 1_274_509_803_922)]
    fn test_part_2(input: &str) -> u64 {
        let gears = Day04::parse(input).unwrap();
        Day04::part_2(&gears)
    }

    #[test_case(EXAMPLE3 => 400)]
    #[test_case(EXAMPLE4 => 6_818)]
    fn test_part_3(input: &str) -> u64 {
        let gears = Day04::parse(input).unwrap();
        Day04::part_3(&gears)
    }
}
