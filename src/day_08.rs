use std::num::ParseIntError;

pub struct Day08;

impl crate::Day for Day08 {
    type Input = Vec<(u16, u16)>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        let sequence: Vec<u16> = input.split(',').map(str::parse).collect::<Result<_, _>>()?;
        let mut sequence: Vec<(u16, u16)> = sequence
            .iter()
            .zip(&sequence[1..])
            .map(|(&x, &y)| (x.min(y), x.max(y)))
            .collect();
        sequence.sort_unstable();
        Ok(sequence)
    }

    fn part_1(input: &Self::Input) -> u64 {
        center_crossings(input, 32)
    }

    fn part_2(input: &Self::Input) -> usize {
        all_crossings(input, 256)
    }

    fn part_3(input: &Self::Input) -> i32 {
        best_cut(input, 256)
    }
}

fn center_crossings(sequence: &[(u16, u16)], nails: u16) -> u64 {
    let mut count = 0;
    for &(x, y) in sequence {
        if x.abs_diff(y) == nails / 2 {
            count += 1;
        }
    }
    count
}

fn all_crossings(sequence: &[(u16, u16)], _nails: usize) -> usize {
    let mut count = 0;
    for (i, &(s1, e1)) in sequence.iter().enumerate() {
        for &(s2, e2) in &sequence[..i] {
            count += usize::from(s2 < s1 && e2 > s1 && e2 < e1);
        }
    }
    count
}

fn best_cut(sequence: &[(u16, u16)], nails: u16) -> i32 {
    let mut count = 0;
    let mut edges = vec![vec![]; nails as usize];
    let mut diffs = vec![0_i32; nails as usize + 1];
    for &(x, y) in sequence {
        edges[x as usize - 1].push(y);
        diffs[x as usize] += 1;
        diffs[y as usize - 1] -= 1;
    }
    for a in 1..nails - 1 {
        for &b in &edges[a as usize - 1] {
            diffs[b as usize - 1] += 2;
            diffs[b as usize] -= 1;
        }
        if a > 1 {
            for &b in &edges[a as usize - 2] {
                diffs[b as usize - 1] -= 1;
                diffs[b as usize] += 2;
            }
        }
        let mut cuts = 0;
        for &add in &diffs[a as usize + 1..nails as usize] {
            cuts += add;
            count = count.max(cuts);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;

    const EXAMPLE1: &str = "1,5,2,6,8,4,1,7,3";
    const EXAMPLE2: &str = "1,5,2,6,8,4,1,7,3,5,7,8,2";
    const EXAMPLE3: &str = "1,5,2,6,8,4,1,7,3,6";

    #[test]
    fn test_part_1() {
        let input = Day08::parse(EXAMPLE1).unwrap();
        let result = center_crossings(&input, 8);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_2() {
        let input = Day08::parse(EXAMPLE2).unwrap();
        let result = all_crossings(&input, 8);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_3() {
        let input = Day08::parse(EXAMPLE3).unwrap();
        let result = best_cut(&input, 8);
        assert_eq!(result, 7);
    }
}
