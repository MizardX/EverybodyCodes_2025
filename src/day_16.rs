use std::cmp::Ordering;
use std::num::ParseIntError;

fn bricks_for_wall_length(spell: &[u64], wall_length: u64) -> u64 {
    spell.iter().map(|&x| wall_length / x).sum()
}

fn spell_for_wall(wall: &[u64]) -> Vec<u64> {
    let mut wall = wall.to_vec();
    let mut spell = Vec::new();
    for i in 0..wall.len() {
        let x = wall[i];
        if x != 0 {
            spell.push(u64::try_from(i + 1).unwrap());
            for j in (i..wall.len()).step_by(i + 1) {
                wall[j] -= 1;
            }
        }
    }
    spell
}

pub struct Day16;

impl crate::Day for Day16 {
    type Input = Vec<u64>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.split(',').map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        bricks_for_wall_length(input, 90)
    }

    fn part_2(input: &Self::Input) -> u64 {
        spell_for_wall(input).into_iter().product()
    }

    fn part_3(input: &Self::Input) -> u64 {
        let spell = spell_for_wall(input);
        let target = 202_520_252_025_000;
        let mut high = 1;
        while bricks_for_wall_length(&spell, high) < target {
            high *= 2;
        }
        let mut low = high / 2;
        while low < high {
            let mid = (low + high).div_ceil(2);
            let res = bricks_for_wall_length(&spell, mid);
            match res.cmp(&target) {
                Ordering::Greater => high = mid - 1,
                Ordering::Equal => return mid,
                Ordering::Less => low = mid,
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_part_1() {
        let input = Day16::parse("1,2,3,5,9").unwrap();
        let result = Day16::part_1(&input);
        assert_eq!(result, 193);
    }

    #[test]
    fn test_part_2() {
        let input = Day16::parse("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2").unwrap();
        let result = Day16::part_2(&input);
        assert_eq!(result, 270);
    }

    #[test]
    fn test_part_3() {
        let input = Day16::parse("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2").unwrap();
        let result = Day16::part_3(&input);
        assert_eq!(result, 94_439_495_762_954);
    }
}
