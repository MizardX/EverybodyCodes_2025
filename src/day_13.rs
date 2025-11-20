use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueRange {
    start: u64,
    end: u64,
}

impl ValueRange {
    const fn len(self) -> u64 {
        self.end - self.start + 1
    }

    fn get(self, ix: u64) -> Option<u64> {
        (self.start + ix <= self.end).then(|| self.start + ix)
    }

    fn get_rev(self, ix: u64) -> Option<u64> {
        (self.start + ix <= self.end).then(|| self.end - ix)
    }
}

impl FromStr for ValueRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((start, end)) = s.split_once('-') {
            Self {
                start: start.parse()?,
                end: end.parse()?,
            }
        } else {
            let val = s.parse()?;
            Self {
                start: val,
                end: val,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Wheel {
    values: Vec<ValueRange>,
}

impl Wheel {
    fn spin(&self, ticks: u64) -> u64 {
        let len = self.values.iter().map(|r| r.len()).sum::<u64>();
        let mut ix = ticks % (len + 1);
        if ix == 0 {
            return 1;
        }
        ix -= 1;
        for rng in self.values.iter().step_by(2) {
            if let Some(res) = rng.get(ix) {
                return res;
            }
            ix -= rng.len();
        }
        for rng in self.values.iter().skip(1).step_by(2).rev() {
            if let Some(res) = rng.get_rev(ix) {
                return res;
            }
            ix -= rng.len();
        }
        unreachable!("Should have been caught by one of the ranges: {ix}")
    }
}

impl FromStr for Wheel {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            values: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

pub struct Day13;

impl crate::Day for Day13 {
    type Input = Wheel;
    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> u64 {
        input.spin(2025)
    }

    fn part_2(input: &Self::Input) -> u64 {
        input.spin(20_252_025)
    }

    fn part_3(input: &Self::Input) -> u64 {
        input.spin(202_520_252_025)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use test_case::test_case;

    #[test_case("72\n58\n47\n61\n67" => 67)]
    #[test_case("72\n58\n47\n61\n67\n2" => 47)]
    #[test_case("72\n58\n47\n61\n67\n2\n3\n4\n5\n6\n7" => 2)]
    #[test_case("72\n58\n47\n61\n67\n2\n3\n4\n5\n6\n7\n8" => 2)]
    fn test_part_1(input: &str) -> u64 {
        let parsed = Day13::parse(input).unwrap();
        Day13::part_1(&parsed)
    }

    const EXAMPLE2: &str = "\
        10-15\n\
        12-13\n\
        20-21\n\
        19-23\n\
        30-37\
    ";

    #[test]
    fn test_part_2() {
        let parsed = Day13::parse(EXAMPLE2).unwrap();
        assert_eq!(Day13::part_2(&parsed), 30);
    }
}
