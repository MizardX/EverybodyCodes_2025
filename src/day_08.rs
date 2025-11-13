use std::collections::HashMap;
use std::num::ParseIntError;

pub struct Day08;

impl crate::Day for Day08 {
    type Input = Vec<u16>;

    type ParseError = ParseIntError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.split(',').map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        center_crossings(input, 32)
    }

    fn part_2(input: &Self::Input) -> u64 {
        all_crossings(input)
    }

    fn part_3(input: &Self::Input) -> impl std::fmt::Display {
        best_cut(input, 256)
    }
}

fn center_crossings(sequence: &[u16], nails: u16) -> u64 {
    let mut count = 0;
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        if x.abs_diff(y) == nails / 2 {
            count += 1;
        }
    }
    count
}

fn all_crossings(sequence: &[u16]) -> u64 {
    let mut count = 0;
    let mut edges = vec![HashMap::<u16, u16>::new(); 256];
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        for z1 in x.min(y) + 1..x.max(y) {
            for (&z2, &cnt) in &edges[z1 as usize - 1] {
                if !(x.min(y)..=x.max(y)).contains(&z2) {
                    count += u64::from(cnt);
                }
            }
        }
        *edges[x as usize - 1].entry(y).or_default() += 1;
        *edges[y as usize - 1].entry(x).or_default() += 1;
    }
    count
}

fn best_cut(sequence: &[u16], nails: u16) -> u64 {
    let mut edges = vec![HashMap::<u16, u16>::new(); 256];
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        *edges[x as usize - 1].entry(y).or_default() += 1;
        *edges[y as usize - 1].entry(x).or_default() += 1;
    }
    let mut max_count = 0;
    for a in 1..nails {
        let mut count = 0;
        for b in a + 2..=nails {
            // How many lines that go exactly from a to b
            let count_coincidents =
                u64::from(edges[b as usize - 1].get(&a).copied().unwrap_or_default());
            // How many lines that goes from b-1 to outside of a..=b
            count += edges[b as usize - 2]
                .iter()
                .filter_map(|(k, v)| (!(a..=b).contains(k)).then_some(u64::from(*v)))
                .sum::<u64>();
            // How many lines that goes from b to inside of a+1..b
            count -= edges[b as usize - 1]
                .iter()
                .filter_map(|(k, v)| ((a + 1..b).contains(k)).then_some(u64::from(*v)))
                .sum::<u64>();
            max_count = max_count.max(count + count_coincidents);
        }
    }
    max_count
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
        let result = all_crossings(&input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_3() {
        let input = Day08::parse(EXAMPLE3).unwrap();
        let result = best_cut(&input, 8);
        assert_eq!(result, 7);
    }
}
