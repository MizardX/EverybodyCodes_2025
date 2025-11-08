use std::cmp::Ordering;
use std::fmt::Debug;
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

#[derive(Clone, PartialEq, Eq)]
pub struct Sword {
    id: u16,
    fishbone: Vec<(Option<u8>, u8, Option<u8>)>,
}

impl Sword {
    const fn len(&self) -> usize {
        self.fishbone.len()
    }

    fn quality(&self) -> u64 {
        self.fishbone.iter().fold(0, |val, &(_, mid, _)| {
            val * if mid < 10 { 10 } else { 100 } + u64::from(mid)
        })
    }

    fn segment(&self, index: usize) -> Option<u32> {
        let (left, mid, right) = self.fishbone.get(index).copied()?;
        let mut value = u32::from(left.unwrap_or(0));
        value = value * 10 + u32::from(mid);
        if let Some(right) = right {
            value = value * 10 + u32::from(right);
        }
        Some(value)
    }
}

impl FromStr for Sword {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, stats) = s.split_once(':').ok_or(ParseError::SyntaxError)?;
        let mut fishbone = Vec::<(Option<u8>, u8, Option<u8>)>::new();
        'stats: for stat in stats.split(',') {
            let stat: u8 = stat.parse()?;
            for (left, mid, right) in &mut fishbone {
                if left.is_none() && stat < *mid {
                    *left = Some(stat);
                    continue 'stats;
                }
                if right.is_none() && stat > *mid {
                    *right = Some(stat);
                    continue 'stats;
                }
            }
            fishbone.push((None, stat, None));
        }
        Ok(Self {
            id: id.parse()?,
            fishbone,
        })
    }
}

impl Debug for Sword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: [", self.id)?;
        for &(left, mid, right) in &self.fishbone {
            match (left, right) {
                (None, None) => write!(f, "(_, {mid}, _)"),
                (Some(left), None) => write!(f, "({left}, {mid}, _)"),
                (None, Some(right)) => write!(f, "(_, {mid}, {right})"),
                (Some(left), Some(right)) => write!(f, "({left}, {mid}, {right})"),
            }?;
        }
        write!(f, "]")
    }
}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.quality().cmp(&other.quality()) {
            Ordering::Equal => (),
            ord => return ord,
        }
        for ix in 0..self.len() {
            match self.segment(ix).cmp(&other.segment(ix)) {
                Ordering::Equal => (),
                ord => return ord,
            }
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day05;

impl crate::Day for Day05 {
    type Input = Vec<Sword>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        input[0].quality()
    }

    fn part_2(input: &Self::Input) -> u64 {
        let mut min = u64::MAX;
        let mut max = u64::MIN;
        for sword in input {
            let spine = sword.quality();
            min = min.min(spine);
            max = max.max(spine);
        }
        max - min
    }

    fn part_3(input: &Self::Input) -> u64 {
        let mut swords = input.clone();
        swords.sort_unstable();
        swords
            .iter()
            .rev()
            .zip(1..)
            .map(|(sword, pos)| pos * u64::from(sword.id))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "58:5,3,7,8,9,10,4,5,7,8,8";

    const EXAMPLE2: &str = "\
        1:2,4,1,1,8,2,7,9,8,6\n\
        2:7,9,9,3,8,3,8,8,6,8\n\
        3:4,7,6,9,1,8,3,7,2,2\n\
        4:6,4,2,1,7,4,5,5,5,8\n\
        5:2,9,3,8,3,9,5,2,1,4\n\
        6:2,4,9,6,7,4,1,7,6,8\n\
        7:2,3,7,6,2,2,4,1,4,2\n\
        8:5,1,5,6,8,3,1,8,3,9\n\
        9:5,7,7,3,7,2,3,8,6,7\n\
        10:4,1,9,3,8,5,4,3,5,5\
    ";

    const EXAMPLE3: &str = "\
        1:7,1,9,1,6,9,8,3,7,2\n\
        2:6,1,9,2,9,8,8,4,3,1\n\
        3:7,1,9,1,6,9,8,3,8,3\n\
        4:6,1,9,2,8,8,8,4,3,1\n\
        5:7,1,9,1,6,9,8,3,7,3\n\
        6:6,1,9,2,8,8,8,4,3,5\n\
        7:3,7,2,2,7,4,4,6,3,1\n\
        8:3,7,2,2,7,4,4,6,3,7\n\
        9:3,7,2,2,7,4,1,6,3,7\
    ";

    const EXAMPLE4: &str = "\
        1:7,1,9,1,6,9,8,3,7,2\n\
        2:7,1,9,1,6,9,8,3,7,2\
    ";

    #[test_case(EXAMPLE1 => 581_078)]
    fn test_part_1(input: &str) -> u64 {
        let input = Day05::parse(input).unwrap();
        Day05::part_1(&input)
    }

    #[test_case(EXAMPLE2 => 77_053)]
    fn test_part_2(input: &str) -> u64 {
        let input = Day05::parse(input).unwrap();
        Day05::part_2(&input)
    }

    #[test_case(EXAMPLE3 => 260)]
    #[test_case(EXAMPLE4 => 4)]
    fn test_part_3(input: &str) -> u64 {
        let input = Day05::parse(input).unwrap();
        Day05::part_3(&input)
    }
}
