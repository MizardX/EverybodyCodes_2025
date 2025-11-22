use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Left(u64),
    Right(u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes()[0] {
            b'L' => Self::Left(s[1..].parse()?),
            b'R' => Self::Right(s[1..].parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    const fn manhattan_dist(self, other: Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
    const fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

impl AddAssign<(Direction, i64)> for Pos {
    fn add_assign(&mut self, (dir, dist): (Direction, i64)) {
        match dir {
            Direction::Up => self.y -= dist,
            Direction::Right => self.x += dist,
            Direction::Down => self.y += dist,
            Direction::Left => self.x -= dist,
        }
    }
}

impl Add<(Direction, i64)> for Pos {
    type Output = Self;

    fn add(mut self, rhs: (Direction, i64)) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, rhs: Direction) {
        *self += (rhs, 1);
    }
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(mut self, rhs: Direction) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<Direction> for Pos {
    fn sub_assign(&mut self, rhs: Direction) {
        *self += (rhs, -1);
    }
}

impl Sub<Direction> for Pos {
    type Output = Self;

    fn sub(mut self, rhs: Direction) -> Self::Output {
        self -= rhs;
        self
    }
}

fn compress_coordinates(instructions: &[Instruction]) -> (Vec<i64>, Vec<i64>) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut pos = Pos::new(0, 0);
    let mut dir = Direction::Up;
    for &instr in instructions {
        let len = match instr {
            Instruction::Left(len) => {
                dir = dir.turn_left();
                len
            }
            Instruction::Right(len) => {
                dir = dir.turn_right();
                len
            }
        };
        let next = pos + (dir, len.cast_signed());
        let before = pos - dir;
        let after = next + dir;
        match dir {
            Direction::Left | Direction::Right => {
                xs.push(before.x);
                xs.push(pos.x);
                xs.push(next.x);
                xs.push(after.x);
                ys.push(pos.y - 1);
                ys.push(pos.y);
                ys.push(pos.y + 1);
            }
            Direction::Up | Direction::Down => {
                xs.push(pos.x - 1);
                xs.push(pos.x);
                xs.push(pos.x + 1);
                ys.push(before.y);
                ys.push(pos.y);
                ys.push(next.y);
                ys.push(after.y);
            }
        }
        pos = next;
    }
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    (xs, ys)
}

fn place_walls(instructions: &[Instruction], xs: &[i64], ys: &[i64]) -> (HashSet<Pos>, Pos) {
    let mut walls = HashSet::new();
    let mut pos = Pos::new(0, 0);
    let mut dir = Direction::Up;

    for &instr in instructions {
        let len = match instr {
            Instruction::Left(len) => {
                dir = dir.turn_left();
                len
            }
            Instruction::Right(len) => {
                dir = dir.turn_right();
                len
            }
        };
        let next = pos + (dir, len.cast_signed());
        match dir {
            Direction::Left | Direction::Right => {
                let y = pos.y;
                let x_ix1 = xs.partition_point(|&x| x < pos.x);
                let x_ix2 = xs.partition_point(|&x| x < next.x);
                for &x in &xs[x_ix1.min(x_ix2)..=x_ix1.max(x_ix2)] {
                    walls.insert(Pos::new(x, y));
                }
            }
            Direction::Up | Direction::Down => {
                let x = pos.x;
                let y_ix1 = ys.partition_point(|&y| y < pos.y);
                let y_ix2 = ys.partition_point(|&y| y < next.y);
                for &y in &ys[y_ix1.min(y_ix2)..=y_ix1.max(y_ix2)] {
                    walls.insert(Pos::new(x, y));
                }
            }
        }
        pos = next;
    }
    walls.remove(&pos);
    (walls, pos)
}

struct Map {
    xs: Vec<i64>,
    ys: Vec<i64>,
    walls: HashSet<Pos>,
}

impl Map {
    const fn new(xs: Vec<i64>, ys: Vec<i64>, walls: HashSet<Pos>) -> Self {
        Self { xs, ys, walls }
    }

    fn djikstra(&self, start: Pos, goal: Pos) -> u64 {
        let mut pending = BinaryHeap::new();
        let mut visited = HashMap::new();
        let start_ixs = (
            self.xs.partition_point(|&x| x < start.x),
            self.ys.partition_point(|&y| y < start.y),
        );
        visited.insert(start, 0);
        pending.push((Reverse(0), start_ixs));
        while let Some((Reverse(dist), (x_ix, y_ix))) = pending.pop() {
            let pos = Pos::new(self.xs[x_ix], self.ys[y_ix]);
            if *visited.get(&pos).expect("all pending should be in visited") < dist {
                continue;
            }
            if pos == goal {
                return dist;
            }
            for (dix_x, dix_y) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let next_ix = (
                    x_ix.wrapping_add_signed(dix_x),
                    y_ix.wrapping_add_signed(dix_y),
                );
                if next_ix.0 < self.xs.len() && next_ix.1 < self.ys.len() {
                    let next = Pos::new(self.xs[next_ix.0], self.ys[next_ix.1]);
                    if !self.walls.contains(&next) {
                        let next_dist = dist + pos.manhattan_dist(next);
                        let old_dist = visited.entry(next).or_insert(u64::MAX);
                        if *old_dist > next_dist {
                            *old_dist = next_dist;
                            pending.push((Reverse(next_dist), next_ix));
                        }
                    }
                }
            }
        }
        0
    }
}

fn find_path(instructions: &[Instruction]) -> u64 {
    let (xs, ys) = compress_coordinates(instructions);
    let (walls, goal) = place_walls(instructions, &xs, &ys);
    Map::new(xs, ys, walls).djikstra(Pos::new(0, 0), goal)
}

pub struct Day15;

impl crate::Day for Day15 {
    type Input = Vec<Instruction>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.split(',').map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> u64 {
        find_path(input)
    }

    fn part_2(input: &Self::Input) -> u64 {
        find_path(input)
    }

    fn part_3(input: &Self::Input) -> u64 {
        find_path(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const EXAMPLE1: &str = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";

    #[test]
    fn test_part_1() {
        let input = Day15::parse(EXAMPLE1).unwrap();
        let result = Day15::part_1(&input);
        assert_eq!(result, 16);
    }
    #[test]
    fn test_part_3() {
        let input = Day15::parse(EXAMPLE1).unwrap();
        let result = Day15::part_3(&input);
        assert_eq!(result, 16);
    }
}
