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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opening {
    ahead: u64,
    start: i64,
    height: i64,
}

impl FromStr for Opening {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let ahead = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let start = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        let height = parts.next().ok_or(ParseError::SyntaxError)?.parse()?;
        if parts.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            ahead,
            start,
            height,
        })
    }
}

fn find_path(input: &[Opening]) -> i64 {
    let mut prev_x = 0;
    let mut prev = vec![(0, 0)];
    let mut next = vec![];
    let mut start_ix = 0;
    let first_wall = input[start_ix];
    let mut end_ix = input.partition_point(|op| op.ahead <= first_wall.ahead);
    while start_ix < input.len() {
        let dx = (input[start_ix].ahead - prev_x).cast_signed();
        prev_x = input[start_ix].ahead;
        for wall in &input[start_ix..end_ix] {
            let y1 = wall.start + (wall.ahead.cast_signed() + wall.start) % 2;
            let y2 = (wall.start + wall.height - 1)
                - (wall.ahead.cast_signed() + wall.start + wall.height - 1) % 2;
            for new_y in (y1..=y2).step_by(2) {
                let mut min_cost = i64::MAX;
                for (y, cost) in &prev {
                    let dy = new_y - y;
                    if dy <= dx && dy >= -dx {
                        let cost1 = cost + i64::midpoint(dx, dy);
                        min_cost = min_cost.min(cost1);
                    }
                }
                if min_cost != i64::MAX {
                    next.push((new_y, min_cost));
                }
            }
        }
        start_ix = end_ix;
        if start_ix == input.len() {
            return next.iter().map(|&(_, cost)| cost).min().unwrap_or(0);
        }
        let first_wall = input[start_ix];
        end_ix = input.partition_point(|op| op.ahead <= first_wall.ahead);
        (prev, next) = (next, prev);
        next.clear();
    }
    0
}

pub struct Day19;

impl crate::Day for Day19 {
    type Input = Vec<Opening>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> i64 {
        find_path(input)
    }

    fn part_2(input: &Self::Input) -> i64 {
        find_path(input)
    }

    fn part_3(input: &Self::Input) -> i64 {
        find_path(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const EXAMPLE1: &str = "\
        7,7,2\n\
        12,0,4\n\
        15,5,3\n\
        24,1,6\n\
        28,5,5\n\
        40,8,2\
    ";

    #[test]
    fn test_part_1() {
        let input = Day19::parse(EXAMPLE1).unwrap();
        let result = Day19::part_1(&input);
        assert_eq!(result, 24);
    }

    const EXAMPLE2: &str = "\
        7,7,2\n\
        7,1,3\n\
        12,0,4\n\
        15,5,3\n\
        24,1,6\n\
        28,5,5\n\
        40,3,3\n\
        40,8,2\
    ";

    #[test]
    fn test_part_2() {
        let input = Day19::parse(EXAMPLE2).unwrap();
        let result = Day19::part_2(&input);
        assert_eq!(result, 22);
    }
}
