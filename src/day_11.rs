use std::num::ParseIntError;

fn phase_1(nums: &mut [u64]) -> bool {
    let mut any_change = false;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            nums[i] -= 1;
            nums[i + 1] += 1;
            any_change = true;
        }
    }
    any_change
}

fn phase_2(nums: &mut [u64]) -> bool {
    let mut any_change = false;
    for i in 0..nums.len() - 1 {
        if nums[i] < nums[i + 1] {
            nums[i] += 1;
            nums[i + 1] -= 1;
            any_change = true;
        }
    }
    any_change
}

fn phase_2_fast(nums: &[u64]) -> u64 {
    let sum = nums.iter().copied().sum::<u64>();
    let avg = sum / u64::try_from(nums.len()).unwrap();
    nums.iter().map(|x| avg.abs_diff(*x)).sum::<u64>() / 2
}

pub struct Day11;

impl crate::Day for Day11 {
    type Input = Vec<u64>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        let mut nums = input.clone();
        let mut turns = 0;
        while turns < 10 && phase_1(&mut nums) {
            turns += 1;
        }
        while turns < 10 && phase_2(&mut nums) {
            turns += 1;
        }
        nums.into_iter().zip(1..).map(|(x, c)| x * c).sum()
    }

    fn part_2(input: &Self::Input) -> u64 {
        let mut nums = input.clone();
        let mut turns = 0;
        while phase_1(&mut nums) {
            turns += 1;
        }
        turns + phase_2_fast(&nums)
    }

    fn part_3(input: &Self::Input) -> u64 {
        phase_2_fast(input)
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
