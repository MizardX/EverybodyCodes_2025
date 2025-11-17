use std::num::ParseIntError;

// use thiserror::Error;

// #[derive(Debug, Error)]
// pub enum ParseError {
//     #[error("Syntax error")]
//     SyntaxError,
//     #[error(transparent)]
//     InvalidNumber(#[from] ParseIntError),
// }

pub struct Day11;

impl crate::Day for Day11 {
    type Input = Vec<u64>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        let mut nums = input.clone();
        let n = nums.len();
        let mut phase = 1;
        for t in 0..10 {
            if phase == 1 {
                let mut any_change = false;
                for i in 0..n - 1 {
                    if nums[i] > nums[i + 1] {
                        nums[i] -= 1;
                        nums[i + 1] += 1;
                        any_change = true;
                    }
                }
                println!("[{t}]: 1st: {nums:?}");
                if any_change {
                    continue;
                }
                phase = 2;
            }
            for i in 0..n - 1 {
                if nums[i] < nums[i + 1] {
                    nums[i] += 1;
                    nums[i + 1] -= 1;
                }
            }
            println!("[{t}]: 2nd: {nums:?}");
        }
        nums.into_iter().zip(1..).map(|(x, c)| x * c).sum()
    }

    fn part_2(input: &Self::Input) -> u64 {
        let mut nums = input.clone();
        let n = nums.len();
        let mut turns = 0;
        let mut phase = 1;
        let mut any_change = true;
        while any_change {
            any_change = false;
            if phase == 1 {
                for i in 0..n - 1 {
                    if nums[i] > nums[i + 1] {
                        nums[i] -= 1;
                        nums[i + 1] += 1;
                        any_change = true;
                    }
                }
                if any_change {
                    turns += 1;
                    continue;
                }
                phase = 2;
            }
            for i in 0..n - 1 {
                if nums[i] < nums[i + 1] {
                    nums[i] += 1;
                    nums[i + 1] -= 1;
                    any_change = true;
                }
            }
            if any_change {
                turns += 1;
            }
        }
        turns
    }

    fn part_3(input: &Self::Input) -> u64 {
        let sum = input.iter().copied().sum::<u64>();
        let avg = sum / u64::try_from(input.len()).unwrap();
        input.iter().map(|x| avg.abs_diff(*x)).sum::<u64>() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use test_case::test_case;

    const EXAMPLE1: &str = "9\n1\n1\n4\n9\n6";
    const EXAMPLE2: &str = "805\n706\n179\n48\n158\n150\n232\n885\n598\n524\n423";

    const EXAMPLE2_SORTED: &str = "48\n150\n158\n179\n232\n423\n524\n598\n706\n805\n885";

    #[test]
    fn test_part_1() {
        let input = Day11::parse(EXAMPLE1).unwrap();
        let result = Day11::part_1(&input);
        assert_eq!(result, 109);
    }

    #[test_case(EXAMPLE1 => 11)]
    #[test_case(EXAMPLE2 => 1579)]
    fn test_part_2(input: &str) -> u64 {
        let input = Day11::parse(input).unwrap();
        Day11::part_2(&input)
    }

    #[test_case(EXAMPLE2_SORTED => 1378)]
    fn test_part_3(input: &str) -> u64 {
        let input = Day11::parse(input).unwrap();
        Day11::part_3(&input)
    }
}
