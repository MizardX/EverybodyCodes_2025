use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error("Invalid tile: {0:?}")]
    InvalidTile(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Active,
    Inactive,
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Inactive,
            b'#' => Self::Active,
            _ => return Err(ParseError::InvalidTile(value as char)),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }

    fn row(&self, r: usize) -> &[T] {
        &self.data[r * self.width..(r + 1) * self.width]
    }

    fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    fn slice_eq(&self, top_left: (usize, usize), other: &Self) -> bool
    where
        T: Eq,
    {
        other
            .rows()
            .enumerate()
            .all(|(r, row)| self.row(top_left.0 + r)[top_left.1..].starts_with(row))
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8, Error = ParseError>,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().ok_or(ParseError::SyntaxError)?.len();
        let mut data = Vec::with_capacity(width * height);
        for row in lines {
            for ch in row.bytes() {
                data.push(ch.try_into()?);
            }
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

fn evolve(grid: &Grid<Tile>, next: &mut Grid<Tile>) {
    for r in 0..grid.height {
        for c in 0..grid.width {
            let mut neighbors = 0;
            if r > 0 {
                if c > 0 && grid[(r - 1, c - 1)] == Tile::Active {
                    neighbors += 1;
                }
                if c + 1 < grid.width && grid[(r - 1, c + 1)] == Tile::Active {
                    neighbors += 1;
                }
            }
            if r + 1 < grid.height {
                if c > 0 && grid[(r + 1, c - 1)] == Tile::Active {
                    neighbors += 1;
                }
                if c + 1 < grid.width && grid[(r + 1, c + 1)] == Tile::Active {
                    neighbors += 1;
                }
            }
            next[(r, c)] = match (grid[(r, c)], neighbors & 1) {
                (Tile::Inactive, 0) | (Tile::Active, 1) => Tile::Active,
                _ => Tile::Inactive,
            };
        }
    }
}

fn simulate(input: &Grid<Tile>, turns: usize) -> usize {
    let mut grid = input.clone();
    let mut next = grid.clone();
    let mut count = 0;
    for _ in 0..turns {
        evolve(&grid, &mut next);
        (next, grid) = (grid, next);
        count += grid
            .data
            .iter()
            .filter(|&&tile| tile == Tile::Active)
            .count();
    }
    count
}

fn simulate_matches(target: &Grid<Tile>, turns: usize) -> usize {
    let mut seen = HashMap::<Grid<Tile>, (usize, usize)>::new();
    let mut grid = Grid::new(vec![Tile::Inactive; 34 * 34], 34, 34);
    let mut next = grid.clone();
    let mut score = 0;
    let top_left = ((34 - target.height) / 2, (34 - target.width) / 2);
    let mut time = 0;
    while time < turns {
        evolve(&grid, &mut next);
        (grid, next) = (next, grid);
        time += 1;

        if grid.slice_eq(top_left, target) {
            score += grid
                .data
                .iter()
                .filter(|&&tile| tile == Tile::Active)
                .count();
        }

        if let Some((prev_time, prev_score)) = seen.get(&grid) {
            let cycle_len = time - prev_time;
            let cycle_value = score - prev_score;
            let remaining_cycles = (turns - time) / cycle_len;
            if remaining_cycles > 0 {
                score += remaining_cycles * cycle_value;
                time += remaining_cycles * cycle_len;
            }
        }

        seen.insert(grid.clone(), (time, score));
    }
    score
}

pub struct Day14;

impl crate::Day for Day14 {
    type Input = Grid<Tile>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> usize {
        simulate(input, 10)
    }

    fn part_2(input: &Self::Input) -> usize {
        simulate(input, 2025)
    }

    fn part_3(input: &Self::Input) -> usize {
        simulate_matches(input, 1_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const EXAMPLE1: &str = "\
        .#.##.\n\
        ##..#.\n\
        ..##.#\n\
        .#.##.\n\
        .###..\n\
        ###.##\
    ";

    const EXAMPLE2: &str = "\
        #......#\n\
        ..#..#..\n\
        .##..##.\n\
        ...##...\n\
        ...##...\n\
        .##..##.\n\
        ..#..#..\n\
        #......#\
    ";

    #[test]
    fn test_part_1() {
        let input = Day14::parse(EXAMPLE1).unwrap();
        let result = Day14::part_1(&input);
        assert_eq!(result, 200);
    }

    #[test]
    fn test_part_3() {
        let input = Day14::parse(EXAMPLE2).unwrap();
        let result = Day14::part_3(&input);
        assert_eq!(result, 278_388_552);
    }
}
