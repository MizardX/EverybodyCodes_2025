use std::collections::VecDeque;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    const fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }
}

impl FromStr for Grid<u8> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().ok_or(ParseError::SyntaxError)?.len();
        let mut data = Vec::with_capacity(width * height);
        for row in lines {
            data.extend_from_slice(row.as_bytes());
        }
        Ok(Self::new(data, width, height))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        if r < self.height && c < self.width {
            &self.data[r * self.width + c]
        } else {
            panic!("Index out of range: {r},{c}");
        }
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        if r < self.height && c < self.width {
            &mut self.data[r * self.width + c]
        } else {
            panic!("Index out of range: {r},{c}");
        }
    }
}

fn fireball(grid: &Grid<u8>, positions: &[(usize, usize)]) -> u64 {
    let mut pending = VecDeque::<(usize, usize)>::new();
    let mut visited = Grid::new(vec![false; grid.data.len()], grid.width, grid.height);
    let mut visited_count = 0;
    for &pos in positions {
        pending.push_back(pos);
        visited[pos] = true;
        visited_count += 1;
    }
    while let Some((r, c)) = pending.pop_front() {
        for (r1, c1) in [
            (r.wrapping_sub(1), c),
            (r, c.wrapping_sub(1)),
            (r + 1, c),
            (r, c + 1),
        ] {
            if r1 < grid.height
                && c1 < grid.width
                && grid[(r1, c1)] <= grid[(r, c)]
                && !visited[(r1, c1)]
            {
                visited[(r1, c1)] = true;
                pending.push_back((r1, c1));
                visited_count += 1;
            }
        }
    }
    visited_count
}

pub struct Day12;

impl crate::Day for Day12 {
    type Input = Grid<u8>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> u64 {
        fireball(input, &[(0, 0)])
    }

    fn part_2(input: &Self::Input) -> u64 {
        fireball(input, &[(0, 0), (input.height - 1, input.width - 1)])
    }

    fn part_3(input: &Self::Input) -> u64 {
        let mut best_score = 0;
        let mut best_pos1 = (0, 0);
        for r in 0..input.height {
            for c in 0..input.width {
                let score = fireball(input, &[(r, c)]);
                if score > best_score {
                    best_score = score;
                    best_pos1 = (r, c);
                }
            }
        }
        let mut best_pos2 = (0, 0);
        for r in 0..input.height {
            for c in 0..input.width {
                let score = fireball(input, &[best_pos1, (r, c)]);
                if score > best_score {
                    best_score = score;
                    best_pos2 = (r, c);
                }
            }
        }
        for r in 0..input.height {
            for c in 0..input.width {
                let score = fireball(input, &[best_pos1, best_pos2, (r, c)]);
                if score > best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Day;

    const EXAMPLE1: &str = "\
        989611\n\
        857782\n\
        746543\n\
        766789\
    ";

    const EXAMPLE2: &str = "\
        9589233445\n\
        9679121695\n\
        8469121876\n\
        8352919876\n\
        7342914327\n\
        7234193437\n\
        6789193538\n\
        6781219648\n\
        5691219769\n\
        5443329859\
    ";

    const EXAMPLE3: &str = "\
        41951111131882511179\n\
        32112222211508122215\n\
        31223333322105122219\n\
        31234444432147511128\n\
        91223333322176021892\n\
        60112222211166431583\n\
        04661111166111111746\n\
        01111119042122222177\n\
        41222108881233333219\n\
        71222127839122222196\n\
        56111026279711111507\
    ";

    #[test]
    fn test_part_1() {
        let input = Day12::parse(EXAMPLE1).unwrap();
        let result = Day12::part_1(&input);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_2() {
        let input = Day12::parse(EXAMPLE2).unwrap();
        let result = Day12::part_2(&input);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part_3() {
        let input = Day12::parse(EXAMPLE3).unwrap();
        let result = Day12::part_3(&input);
        assert_eq!(result, 133);
    }
}
