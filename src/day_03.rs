use std::num::ParseIntError;

use crate::Day;

pub struct Day03;

impl Day for Day03 {
    type Input = Vec<u16>;
    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        let mut result = input
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        result.sort_unstable(); // Sort here, to avoid unnessesary cloning
        Ok(result)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut largest_set = 0;
        for size_group in input.chunk_by(PartialEq::eq) {
            // SAFETY: chunk_by yields groups of size at least one.
            let &size = unsafe { size_group.get_unchecked(0) };
            largest_set += usize::from(size);
        }
        largest_set
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut smallest_set = 0;
        for size_group in input.chunk_by(PartialEq::eq).take(20) {
            // SAFETY: chunk_by yields groups of size at least one.
            let &size = unsafe { size_group.get_unchecked(0) };
            smallest_set += usize::from(size);
        }
        smallest_set
    }

    type Output3 = usize;

    fn part_3(input: &Self::Input) -> Self::Output3 {
        input
            .chunk_by(PartialEq::eq)
            .map(<[_]>::len)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "10,5,1,10,3,8,5,2,2";
    const EXAMPLE2: &str = "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";

    #[test]
    fn test_part_1() {
        let input = Day03::parse(EXAMPLE1).unwrap();
        let result = Day03::part_1(&input);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_part_2() {
        let input = Day03::parse(EXAMPLE2).unwrap();
        let result = Day03::part_2(&input);
        assert_eq!(result, 781);
    }

    #[test]
    fn test_part_3() {
        let input = Day03::parse(EXAMPLE2).unwrap();
        let result = Day03::part_3(&input);
        assert_eq!(result, 3);
    }
}
