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

    fn part_2(input: &Self::Input) -> usize {
        all_crossings(input, 256)
    }

    fn part_3(input: &Self::Input) -> usize {
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

fn all_crossings(sequence: &[u16], nails: usize) -> usize {
    let mut count = 0;
    let mut edges = vec![vec![]; nails];
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        edges[x as usize - 1].push(y);
        edges[y as usize - 1].push(x);
    }
    for e in &mut edges {
        e.sort_unstable();
    }
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        let low = x.min(y);
        let high = x.max(y);
        for z in low + 1..high {
            count += edges[z as usize - 1].len()
                - binary_count_range(&edges[z as usize - 1], &low, &high);
        }
    }
    count / 2
}

fn best_cut(sequence: &[u16], nails: u16) -> usize {
    let mut edges = vec![vec![]; usize::from(nails)];
    for (&x, &y) in sequence.iter().zip(&sequence[1..]) {
        edges[x as usize - 1].push(y);
        edges[y as usize - 1].push(x);
    }
    for e in &mut edges {
        e.sort_unstable();
    }
    let mut max_count = 0;
    for a in 1..nails {
        let mut count = 0;
        for b in a + 2..=nails {
            // How many lines that go exactly from a to b
            let count_coincidents = binary_count_range(&edges[b as usize - 1], &a, &a);
            // How many lines that goes from b-1 to outside of a..=b
            count +=
                edges[b as usize - 2].len() - binary_count_range(&edges[b as usize - 2], &a, &b);
            // How many lines that goes from b to inside of a+1..b
            count -= binary_count_range(&edges[b as usize - 1], &(a + 1), &(b - 1));
            max_count = max_count.max(count + count_coincidents);
        }
    }
    max_count
}

fn binary_count_range<T: Ord>(sorted: &[T], min: &T, max: &T) -> usize {
    sorted.partition_point(|x| x <= max) - sorted.partition_point(|x| x < min)
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
