use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: u8,
    col: u8,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row: u8::try_from(row).unwrap(),
            col: u8::try_from(col).unwrap(),
        }
    }

    fn sub_row(self, rows: usize) -> Option<Self> {
        self.row
            .checked_sub(u8::try_from(rows).unwrap())
            .map(|row| Self { row, ..self })
    }

    const fn into_index(self, width: usize) -> usize {
        self.row as usize * width + self.col as usize
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (b'A' + self.col) as char, self.row + 1)
    }
}

#[derive(Debug, Clone)]
struct DragonMoves {
    origin: Pos,
    width: usize,
    height: usize,
    index: usize,
}

impl DragonMoves {
    const fn new(origin: Pos, width: usize, height: usize) -> Self {
        Self {
            origin,
            width,
            height,
            index: 0,
        }
    }
}

impl Iterator for DragonMoves {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        const MOVES: [(i8, i8); 8] = [
            (-1, -2),
            (1, -2),
            (-2, -1),
            (2, -1),
            (-2, 1),
            (2, 1),
            (-1, 2),
            (1, 2),
        ];
        while self.index < 8 {
            if let Some(c1) = self.origin.col.checked_add_signed(MOVES[self.index].0)
                && usize::from(c1) < self.width
                && let Some(r1) = self.origin.row.checked_add_signed(MOVES[self.index].1)
                && usize::from(r1) < self.height
            {
                self.index += 1;
                return Some(Pos { row: r1, col: c1 });
            }
            self.index += 1;
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: usize,
    height: usize,
    dragon: Pos,
    sheep: Vec<bool>,
    blocked: Vec<bool>,
}

impl Board {
    fn has_sheep_at(&self, pos: Pos) -> bool {
        self.sheep[pos.into_index(self.width)]
    }

    fn is_blocked(&self, pos: Pos) -> bool {
        self.blocked[pos.into_index(self.width)]
    }

    const fn dragon_moves(&self, dragon: Pos) -> DragonMoves {
        DragonMoves::new(dragon, self.width, self.height)
    }
}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let width = lines.clone().next().ok_or(ParseError::SyntaxError)?.len();
        let height = lines.clone().count();
        let mut dragon = None;
        let mut sheep = vec![false; width * height];
        let mut blocked = vec![false; width * height];
        for (r, row) in lines.enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                let pos = Pos::new(r, c);
                match ch {
                    b'.' => (),
                    b'D' if dragon.is_none() => dragon = Some(Pos::new(r, c)),
                    b'S' => sheep[pos.into_index(width)] = true,
                    b'#' => blocked[pos.into_index(width)] = true,
                    _ => return Err(ParseError::SyntaxError),
                }
            }
        }
        Ok(Self {
            width,
            height,
            dragon: dragon.ok_or(ParseError::SyntaxError)?,
            sheep,
            blocked,
        })
    }
}

#[derive(Debug, Clone)]
struct StaticSheep<'a> {
    board: &'a Board,
    visited: Vec<bool>,
    captured_sheep: Vec<bool>,
}

impl<'a> StaticSheep<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            visited: vec![false; board.width * board.height],
            captured_sheep: vec![false; board.width * board.height],
        }
    }

    fn has_static_sheep_at(&self, pos: Pos) -> bool {
        self.board.has_sheep_at(pos) && !self.captured_sheep[pos.into_index(self.board.width)]
    }

    fn set_captued(&mut self, pos: Pos) {
        self.captured_sheep[pos.into_index(self.board.width)] = true;
    }

    fn has_visited(&self, pos: Pos) -> bool {
        self.visited[pos.into_index(self.board.width)]
    }

    fn reachable_static_sheep(&mut self, max_dist: usize) -> usize {
        let mut pending = vec![self.board.dragon];
        let mut next = Vec::new();
        self.visited.fill(false);
        let mut reachable = 0;
        for _ in 0..=max_dist {
            for &pos in &pending {
                let vis = &mut self.visited[pos.into_index(self.board.width)];
                if *vis {
                    continue;
                }
                *vis = true;

                if self.has_static_sheep_at(pos) {
                    self.set_captued(pos);
                    reachable += 1;
                }

                for pos1 in self.board.dragon_moves(pos) {
                    if !self.has_visited(pos1) {
                        next.push(pos1);
                    }
                }
            }
            println!("{self}");
            (next, pending) = (pending, next);
            next.clear();
        }
        reachable
    }
}

impl Display for StaticSheep<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.board.height {
            writeln!(f)?;
            for c in 0..self.board.width {
                let pos = Pos::new(r, c);
                if self.has_visited(pos) {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "  ")?;
            for c in 0..self.board.width {
                let pos = Pos::new(r, c);
                let sheep = self.board.has_sheep_at(pos);
                let alive_sheep = self.has_static_sheep_at(pos);
                match (sheep, alive_sheep) {
                    (true, false) => write!(f, "\x1b[1;31mX\x1b[0m")?,
                    (true, _) => write!(f, "S")?,
                    _ => write!(f, ".")?,
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct DynamicSheep<'a> {
    board: &'a Board,
    visited: Vec<bool>,
    captured_sheep: Vec<Option<usize>>,
    time: usize,
}

impl<'a> DynamicSheep<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            visited: vec![false; board.width * board.height],
            captured_sheep: vec![None; board.width * board.height],
            time: 0,
        }
    }

    fn has_moving_sheep_at(&self, pos: Pos, time: usize) -> bool {
        pos.sub_row(time).is_some_and(|pos1| {
            self.board.has_sheep_at(pos1)
                && self.captured_sheep[pos1.into_index(self.board.width)].is_none()
        })
    }

    fn was_captured_at(&self, pos: Pos, sheep_time: usize, dragon_time: usize) -> bool {
        pos.sub_row(sheep_time).is_some_and(|pos1| {
            self.board.has_sheep_at(pos1)
                && self.captured_sheep[pos1.into_index(self.board.width)] == Some(dragon_time)
        })
    }

    fn was_captured_any_time(&self, pos: Pos, sheep_time: usize) -> bool {
        pos.sub_row(sheep_time).is_some_and(|pos1| {
            self.board.has_sheep_at(pos1)
                && self.captured_sheep[pos1.into_index(self.board.width)].is_some()
        })
    }

    fn set_captued(&mut self, pos: Pos, time: usize) {
        if let Some(pos1) = pos.sub_row(time) {
            assert!(self.board.has_sheep_at(pos1));
            self.captured_sheep[pos1.into_index(self.board.width)] = Some(self.time);
        }
    }

    fn has_visited(&self, pos: Pos) -> bool {
        self.visited[pos.into_index(self.board.width)]
    }

    fn reachable_moving_sheep(&mut self, max_dist: usize) -> usize {
        let mut pending = self
            .board
            .dragon_moves(self.board.dragon)
            .collect::<Vec<_>>();
        let mut next = Vec::new();
        self.visited.fill(false);
        self.captured_sheep.fill(None);
        let mut reachable = 0;
        for time in 0..max_dist {
            self.time = time;
            for &pos in &pending {
                let vis = &mut self.visited[pos.into_index(self.board.width)];
                if *vis {
                    continue;
                }
                *vis = true;

                for pos1 in self.board.dragon_moves(pos) {
                    if !self.visited[pos1.into_index(self.board.width)] {
                        next.push(pos1);
                    }
                }
            }
            for r in 0..self.board.height {
                for c in 0..self.board.width {
                    let parity =
                        (r + c + time) + usize::from(self.board.dragon.row + self.board.dragon.col);
                    if parity & 1 == 0 {
                        continue;
                    }
                    for t in 0..=1 {
                        let pos1 = Pos::new(r, c);
                        let time1 = time + t;
                        if self.has_visited(pos1)
                            && self.has_moving_sheep_at(pos1, time1)
                            && !self.board.is_blocked(pos1)
                        {
                            self.set_captued(pos1, time1);
                            reachable += 1;
                        }
                    }
                }
            }
            println!("{self}");
            (next, pending) = (pending, next);
            next.clear();
        }
        reachable
    }
}

impl Display for DynamicSheep<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.board.height {
            writeln!(f)?;
            for c in 0..self.board.width {
                let pos = Pos::new(r, c);
                let parity = (r + c + self.time)
                    + usize::from(self.board.dragon.row + self.board.dragon.col);
                if self.has_visited(pos) && parity & 1 == 1 {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            for time1 in self.time..=self.time + 1 {
                write!(f, "  ")?;
                for c in 0..self.board.width {
                    let pos1 = Pos::new(r, c);
                    let sheep = self.has_moving_sheep_at(pos1, time1);
                    let captured_now = self.was_captured_at(pos1, time1, self.time);
                    let captured_any = self.was_captured_any_time(pos1, time1);
                    let parity = (r + c + self.time)
                        + usize::from(self.board.dragon.row + self.board.dragon.col);
                    let blocked = self.board.is_blocked(pos1);
                    let visited = self.has_visited(pos1) && parity & 1 == 1;
                    match (
                        sheep || captured_any,
                        captured_now && parity & 1 == 1,
                        blocked,
                        visited,
                    ) {
                        (true, _, true, true) => write!(f, "\x1b[1;32mS\x1b[0m")?,
                        (_, true, _, _) => write!(f, "\x1b[1;31mX\x1b[0m")?,
                        (true, _, _, _) => write!(f, "S")?,
                        _ => write!(f, ".")?,
                    }
                }
            }
            write!(f, "  ")?;
            for c in 0..self.board.width {
                let pos1 = Pos::new(r, c);
                let blocked = self.board.is_blocked(pos1);
                let visited = self.has_visited(pos1);
                match (visited, blocked) {
                    (true, true) => write!(f, "\x1b[1;32m#\x1b[0m")?,
                    (_, true) => write!(f, "#")?,
                    _ => write!(f, ".")?,
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Game<'a> {
    board: &'a Board,
    dragon: Pos,
    sheep: Vec<u8>,
    safe: Vec<u8>,
}

type CacheKey = (bool, Pos, Vec<u8>);

impl<'a> Game<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            dragon: board.dragon,
            sheep: (0..board.width)
                .map(|c| {
                    (0..board.height)
                        .find(|&r| board.has_sheep_at(Pos::new(r, c)))
                        .map_or(99, |r| u8::try_from(r).unwrap())
                })
                .collect(),
            safe: (0..board.width)
                .map(|c| {
                    (1..=board.height)
                        .rev()
                        .find(|r| !board.is_blocked(Pos::new(r - 1, c)))
                        .map_or(99, |r| u8::try_from(r).unwrap())
                })
                .collect(),
        }
    }

    fn cache_key(&self, dragon_turn: bool) -> CacheKey {
        (dragon_turn, self.dragon, self.sheep.clone())
    }

    fn count_winning_games(&mut self) -> usize {
        let mut seen = HashMap::new();
        self.sheep_moves(&mut seen)
    }

    fn has_sheep_at(&self, pos: Pos) -> bool {
        self.sheep[usize::from(pos.col)] == pos.row
    }

    fn is_safe(&self, pos: Pos) -> bool {
        self.safe[usize::from(pos.col)] <= pos.row
    }

    fn dragon_moves(&mut self, seen: &mut HashMap<CacheKey, usize>) -> usize {
        let cache_key = self.cache_key(true);
        if let Some(&cached) = seen.get(&cache_key) {
            return cached;
        }
        let pos = self.dragon;
        let mut count = 0;
        for pos1 in self.board.dragon_moves(pos) {
            self.dragon = pos1;
            if !self.board.is_blocked(pos1) && self.has_sheep_at(pos1) {
                self.sheep[usize::from(pos1.col)] = 99;
                count += self.sheep_moves(seen);
                self.sheep[usize::from(pos1.col)] = pos1.row;
            } else {
                count += self.sheep_moves(seen);
            }
        }
        self.dragon = pos;
        seen.insert(cache_key, count);
        count
    }

    fn sheep_moves(&mut self, seen: &mut HashMap<CacheKey, usize>) -> usize {
        let cache_key = self.cache_key(false);
        if let Some(&cached) = seen.get(&cache_key) {
            return cached;
        }
        if self.sheep.iter().all(|&r| r == 99) {
            return 1;
        }
        let mut count = 0;
        let mut any_move = false;
        for c in 0..self.board.width {
            let r = self.sheep[c];
            if r == 99 {
                continue;
            }
            let r1 = r + 1;
            let pos1 = Pos::new(usize::from(r1), c);
            if Pos::new(usize::from(r1), c) == self.dragon && !self.board.is_blocked(pos1) {
                continue;
            }
            // From the sheeps perspective, the only illegal move is to move into the dragon.
            // So for the purpuse of allowing double moves, only count this one, even every other
            // move leads to sheeps escaping.
            any_move = true;
            if usize::from(r1) == self.board.height || self.is_safe(pos1) {
                continue;
            }
            self.sheep[c] = r1;
            count += self.dragon_moves(seen);
            self.sheep[c] = r;
        }
        if !any_move {
            // Double move
            count += self.dragon_moves(seen);
        }
        seen.insert(cache_key, count);
        count
    }
}

impl Display for Game<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.board.height {
            writeln!(f)?;
            for c in 0..self.board.width {
                let pos = Pos::new(r, c);
                if self.dragon == pos {
                    write!(f, "D")?;
                } else if self.has_sheep_at(pos) {
                    if self.board.is_blocked(pos) {
                        write!(f, "\x1b[1;32m#\x1b[0m")?;
                    } else {
                        write!(f, "s")?;
                    }
                } else if self.board.is_blocked(pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

pub struct Day10;

impl crate::Day for Day10 {
    type Input = Board;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> usize {
        StaticSheep::new(input).reachable_static_sheep(4)
    }

    fn part_2(input: &Self::Input) -> usize {
        DynamicSheep::new(input).reachable_moving_sheep(20)
    }

    fn part_3(input: &Self::Input) -> usize {
        Game::new(input).count_winning_games()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use test_case::test_case;

    const P1_EXAMPLE: &str = "\
        ...SSS.......\n\
        .S......S.SS.\n\
        ..S....S...S.\n\
        ..........SS.\n\
        ..SSSS...S...\n\
        .....SS..S..S\n\
        SS....D.S....\n\
        S.S..S..S....\n\
        ....S.......S\n\
        .SSS..SS.....\n\
        .........S...\n\
        .......S....S\n\
        SS.....S..S..\
    ";

    const P2_EXAMPLE: &str = "\
        ...SSS##.....\n\
        .S#.##..S#SS.\n\
        ..S.##.S#..S.\n\
        .#..#S##..SS.\n\
        ..SSSS.#.S.#.\n\
        .##..SS.#S.#S\n\
        SS##.#D.S.#..\n\
        S.S..S..S###.\n\
        .##.S#.#....S\n\
        .SSS.#SS..##.\n\
        ..#.##...S##.\n\
        .#...#.S#...S\n\
        SS...#.S.#S..\
    ";

    const P3_EXAMPLE1: &str = "\
        SSS\n\
        ..#\n\
        #.#\n\
        #D.\
    ";

    const P3_EXAMPLE2: &str = "\
        SSS\n\
        ..#\n\
        ..#\n\
        .##\n\
        .D#\
    ";

    const P3_EXAMPLE3: &str = "\
        ..S..\n\
        .....\n\
        ..#..\n\
        .....\n\
        ..D..\
    ";

    const P3_EXAMPLE4: &str = "\
        .SS.S\n\
        #...#\n\
        ...#.\n\
        ##..#\n\
        .####\n\
        ##D.#\
    ";

    const P3_EXAMPLE5: &str = "\
        SSS.S\n\
        .....\n\
        #.#.#\n\
        .#.#.\n\
        #.D.#\
    ";

    #[test]
    fn test_part_1() {
        let board = Day10::parse(P1_EXAMPLE).unwrap();
        let result = StaticSheep::new(&board).reachable_static_sheep(3);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_part_2() {
        let board = Day10::parse(P2_EXAMPLE).unwrap();
        let result = DynamicSheep::new(&board).reachable_moving_sheep(3);
        assert_eq!(result, 27);
    }

    #[test_case(P3_EXAMPLE1 => 15)]
    #[test_case(P3_EXAMPLE2 => 8)]
    #[test_case(P3_EXAMPLE3 => 44)]
    #[test_case(P3_EXAMPLE4 => 4_406)]
    #[test_case(P3_EXAMPLE5 => 13_033_988_838)]
    fn test_part_3(input: &str) -> usize {
        let board = Day10::parse(input).unwrap();
        Game::new(&board).count_winning_games()
    }
}
