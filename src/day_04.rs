use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    Single(u64),
    Double(u64, u64),
}

impl FromStr for Gear {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once('|') {
            Ok(Self::Double(left.parse()?, right.parse()?))
        } else {
            Ok(Self::Single(s.parse()?))
        }
    }
}

pub struct Day04;

impl crate::Day for Day04 {
    type Input = Vec<Gear>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    type Output1 = u64;
    fn part_1(gears: &Self::Input) -> Self::Output1 {
        let &[Gear::Single(first), .., Gear::Single(last)] = gears.as_slice() else {
            panic!("Input should start and end with a single gear")
        };
        first * 2025 / last
    }

    type Output2 = u64;
    fn part_2(gears: &Self::Input) -> Self::Output2 {
        let &[Gear::Single(first), .., Gear::Single(last)] = gears.as_slice() else {
            panic!("Input should start and end with a single gear")
        };
        (10_000_000_000_000 * last).div_ceil(first)
    }

    type Output3 = u64;
    fn part_3(gears: &Self::Input) -> Self::Output3 {
        let &[Gear::Single(first), ref shifts @ .., Gear::Single(last)] = gears.as_slice() else {
            panic!("Input should start and end with a single gear")
        };
        let mut teeth = 100 * first;
        for &gear in shifts {
            let Gear::Double(left, right) = gear else {
                panic!("Input should only contain doubles in between first and last")
            };
            teeth = teeth * right / left;
        }
        teeth / last
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
