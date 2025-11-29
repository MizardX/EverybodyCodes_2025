use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Write};
use std::ops::{Add, Index};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    UpRight,
    DownRight,
    Down,
    DownLeft,
    UpLeft,
}

impl Direction {
    const fn all() -> [Self; 6] {
        [
            Self::Up,
            Self::UpRight,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::UpLeft,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Parity {
    L,
    R,
}

impl Parity {
    const fn flip(&mut self) {
        match *self {
            Self::L => *self = Self::R,
            Self::R => *self = Self::L,
        }
    }

    const fn into_index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    q: usize,
    r: usize,
    parity: Parity,
}

impl Pos {
    const fn new(q: usize, r: usize, parity: Parity) -> Self {
        Self { q, r, parity }
    }

    const fn into_index(self, size: usize) -> usize {
        2 * self.q + self.parity.into_index() + self.r * (2 * size - self.r)
    }

    const fn within_grid(self, size: usize) -> bool {
        self.r < size
            && (self.q < size - self.r - 1
                || self.q == size - self.r - 1 && matches!(self.parity, Parity::L))
    }

    const fn rotate_ccw(self, size: usize) -> Self {
        Self {
            q: self.r,
            r: size - 1 - self.parity.into_index() - self.q - self.r,
            parity: self.parity,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}{:?}", self.q, self.r, self.parity)
    }
}

impl Add<Direction> for Pos {
    type Output = Option<Self>;

    fn add(mut self, rhs: Direction) -> Self::Output {
        match (self.parity, rhs) {
            (Parity::L, Direction::Up) if self.r > 0 => self.r -= 1,
            (Parity::L, Direction::DownRight) | (Parity::R, Direction::UpLeft) => {}
            (Parity::L, Direction::DownLeft) if self.q > 0 => self.q -= 1,
            (Parity::R, Direction::UpRight) => self.q += 1,
            (Parity::R, Direction::Down) => self.r += 1,
            _ => return None,
        }
        self.parity.flip();
        Some(self)
    }
}

pub struct TriangularGrid<T> {
    data: Vec<T>,
    /// triangle side length
    size: usize,
}

impl<T> TriangularGrid<T> {
    #[allow(unused, reason = "only used in tests")]
    pub fn new(data: Vec<T>, size: usize) -> Self {
        assert_eq!(data.len(), size * size, "data.len() == size * size");
        Self { data, size }
    }

    fn positions(&self) -> impl Iterator<Item = Pos> {
        (0..self.size).flat_map(|r| {
            (0..self.size - r - 1)
                .flat_map(move |q| [Pos::new(q, r, Parity::L), Pos::new(q, r, Parity::R)])
                .chain([Pos::new(self.size - r - 1, r, Parity::L)])
        })
    }
}

impl<T> Index<Pos> for TriangularGrid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        if index.q + index.r + 1 < self.size
            || index.q + index.r + 1 == self.size && index.parity == Parity::L
        {
            &self.data[index.into_index(self.size)]
        } else {
            panic!("Index out of range: {index:?} on size {} grid", self.size)
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("InvalidTile")]
    InvalidTile,
    #[error("Grid is not triangular")]
    ShapeError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Hole,
    Trampoline,
    Start,
    End,
}

impl Tile {
    const fn is_passable(self) -> bool {
        !matches!(self, Self::Hole)
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'#' => Self::Hole,
            b'T' => Self::Trampoline,
            b'S' => Self::Start,
            b'E' => Self::End,
            _ => return Err(ParseError::InvalidTile),
        })
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Hole => '#',
            Self::Trampoline => 'T',
            Self::Start => 'S',
            Self::End => 'E',
        })
    }
}

impl FromStr for TriangularGrid<Tile> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let size = lines.clone().count();
        let mut data = Vec::new();
        for (r, row) in lines.enumerate() {
            if row.len() != size * 2 - 1 {
                return Err(ParseError::ShapeError);
            }
            if row[..r]
                .bytes()
                .chain(row[2 * size - 1 - r..].bytes())
                .any(|b| b != b'.')
            {
                return Err(ParseError::ShapeError);
            }
            for ch in row[r..2 * size - 1 - r].bytes() {
                data.push(Tile::try_from(ch)?);
            }
        }
        Ok(Self { data, size })
    }
}

/*
enum Rotated<'a> {
    Normal(&'a TriangularGrid<Tile>, Pos),
    Clockwise(&'a TriangularGrid<Tile>, Pos),
    CounterClockwise(&'a TriangularGrid<Tile>, Pos),
}

impl Rotated<'_> {
    const fn rotate_cw(self) -> Self {
        match self {
            Self::Normal(grid, pos) => Self::Clockwise(grid, pos),
            Self::Clockwise(grid, pos) => Self::CounterClockwise(grid, pos),
            Self::CounterClockwise(grid, pos) => Self::Normal(grid, pos),
        }
    }
}

impl Display for Rotated<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (grid, pos1, transform): (_, _, fn(Pos, usize) -> Pos) = match *self {
            Self::Normal(grid, pos1) => (grid, pos1, |pos, _| pos),
            Self::Clockwise(grid, pos1) => (grid, pos1, |pos, size| pos.rotate_ccw(size)),
            Self::CounterClockwise(grid, pos1) => (grid, pos1, |pos, size| pos.rotate_cw(size)),
        };
        for r in 0..grid.size {
            write!(f, "{:.<r$}", "")?;
            for q in 0..grid.size - r - 1 {
                let pos = transform(Pos::new(q, r, Parity::L), grid.size);
                if pos == pos1 {
                    write!(f, "\x1b[1;92m{}\x1b[0m", grid[pos])?;
                } else {
                    write!(f, "{}", grid[pos])?;
                }
                let pos = transform(Pos::new(q, r, Parity::R), grid.size);
                if pos == pos1 {
                    write!(f, "\x1b[1;92m{}\x1b[0m", grid[pos])?;
                } else {
                    write!(f, "{}", grid[pos])?;
                }
            }
            let pos = transform(Pos::new(grid.size - r - 1, r, Parity::L), grid.size);
            if pos == pos1 {
                writeln!(f, "\x1b[1;92m{}\x1b[0m{:.<r$}", grid[pos], "")?;
            } else {
                writeln!(f, "{}{:.<r$}", grid[pos], "")?;
            }
        }
        Ok(())
    }
}
*/

fn find_path<M, MI>(input: &TriangularGrid<Tile>, moves: M) -> u64
where
    M: Fn(Pos) -> MI,
    MI: Iterator<Item = Pos>,
{
    let start = input
        .positions()
        .find(|&pos| input[pos] == Tile::Start)
        .expect("Start position on grid");
    let end = input
        .positions()
        .find(|&pos| input[pos] == Tile::End)
        .expect("End position on grid");
    let mut pending = VecDeque::new();
    pending.push_back((start, 0));
    let mut visited = HashSet::new();
    visited.insert(start);
    while let Some((pos, dist)) = pending.pop_front() {
        if pos == end {
            return dist;
        }
        for next in moves(pos) {
            if input[next].is_passable() && visited.insert(next) {
                pending.push_back((next, dist + 1));
            }
        }
    }
    0
}

pub struct Day20;

impl crate::Day for Day20 {
    type Input = TriangularGrid<Tile>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> usize {
        input
            .positions()
            .filter(|&pos| input[pos] == Tile::Trampoline)
            .map(|pos| {
                Direction::all()
                    .into_iter()
                    .filter_map(|dir| pos + dir)
                    .filter(|&pos1| pos1.within_grid(input.size) && input[pos1] == Tile::Trampoline)
                    .count()
            })
            .sum::<usize>()
            / 2
    }

    fn part_2(input: &Self::Input) -> u64 {
        find_path(input, |pos| {
            Direction::all().into_iter().filter_map(move |dir| pos + dir)
        })
    }

    fn part_3(input: &Self::Input) -> u64 {
        find_path(input, |pos| {
            Direction::all()
                .into_iter()
                .filter_map(move |dir| pos + dir)
                .filter(|&pos| pos.within_grid(input.size))
                .chain([pos])
                .map(|pos| pos.rotate_ccw(input.size))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use test_case::test_case;

    #[test]
    fn test_triangle_positions() {
        let positons: Vec<Pos> = TriangularGrid::<()>::new(vec![(); 3 * 3], 3)
            .positions()
            .collect();
        assert_eq!(
            positons,
            [
                Pos::new(0, 0, Parity::L),
                Pos::new(0, 0, Parity::R),
                Pos::new(1, 0, Parity::L),
                Pos::new(1, 0, Parity::R),
                Pos::new(2, 0, Parity::L),
                Pos::new(0, 1, Parity::L),
                Pos::new(0, 1, Parity::R),
                Pos::new(1, 1, Parity::L),
                Pos::new(0, 2, Parity::L),
            ]
        );
    }

    #[test]
    fn test_rotation() {
        let positons: Vec<Pos> = TriangularGrid::<()>::new(vec![(); 3 * 3], 3)
            .positions()
            .map(|pos| pos.rotate_ccw(3))
            .collect();
        assert_eq!(
            positons,
            [
                Pos::new(0, 2, Parity::L),
                Pos::new(0, 1, Parity::R),
                Pos::new(0, 1, Parity::L),
                Pos::new(0, 0, Parity::R),
                Pos::new(0, 0, Parity::L),
                Pos::new(1, 1, Parity::L),
                Pos::new(1, 0, Parity::R),
                Pos::new(1, 0, Parity::L),
                Pos::new(2, 0, Parity::L),
            ]
        );
    }

    const EXAMPLE1_A: &str = "\
        T#TTT###T##\n\
        .##TT#TT##.\n\
        ..T###T#T..\n\
        ...##TT#...\n\
        ....T##....\n\
        .....#.....\
    ";

    const EXAMPLE1_B: &str = "\
        T#T#T#T#T#T\n\
        .T#T#T#T#T.\n\
        ..T#T#T#T..\n\
        ...T#T#T...\n\
        ....T#T....\n\
        .....T.....\
    ";

    const EXAMPLE1_C: &str = "\
        T#T#T#T#T#T\n\
        .#T#T#T#T#.\n\
        ..#T###T#..\n\
        ...##T##...\n\
        ....#T#....\n\
        .....#.....\
    ";

    #[test_case(EXAMPLE1_A => 7)]
    #[test_case(EXAMPLE1_B => 0)]
    #[test_case(EXAMPLE1_C => 0)]
    fn test_part_1(input: &str) -> usize {
        let triangles = Day20::parse(input).unwrap();
        Day20::part_1(&triangles)
    }

    const EXAMPLE2: &str = "\
        TTTTTTTTTTTTTTTTT\n\
        .TTTT#T#T#TTTTTT.\n\
        ..TT#TTTETT#TTT..\n\
        ...TT#T#TTT#TT...\n\
        ....TTT#T#TTT....\n\
        .....TTTTTT#.....\n\
        ......TT#TT......\n\
        .......#TT.......\n\
        ........S........\
    ";

    #[test]
    fn test_part_2() {
        let grid = Day20::parse(EXAMPLE2).unwrap();
        let result = Day20::part_2(&grid);
        assert_eq!(result, 32);
    }

    const EXAMPLE3: &str = "\
        T####T#TTT##T##T#T#\n\
        .T#####TTTT##TTT##.\n\
        ..TTTT#T###TTTT#T..\n\
        ...T#TTT#ETTTT##...\n\
        ....#TT##T#T##T....\n\
        .....#TT####T#.....\n\
        ......T#TT#T#......\n\
        .......T#TTT.......\n\
        ........TT#........\n\
        .........S.........\
    ";

    #[test]
    fn test_part_3() {
        let grid = Day20::parse(EXAMPLE3).unwrap();
        let result = Day20::part_3(&grid);
        assert_eq!(result, 23);
    }
}
