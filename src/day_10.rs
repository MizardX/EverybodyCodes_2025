use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    width: usize,
    height: usize,
    dragon: (usize, usize),
    sheep: Vec<bool>,
    blocked: Vec<bool>,
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
                match ch {
                    b'.' => (),
                    b'D' if dragon.is_none() => dragon = Some((r, c)),
                    b'S' => sheep[r * width + c] = true,
                    b'#' => blocked[r * width + c] = true,
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
struct QuantumDragon<'a> {
    board: &'a Board,
    visited: Vec<bool>,
    captured_sheep: Vec<Option<usize>>,
    time: usize,
}

impl<'a> QuantumDragon<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            visited: vec![false; board.width * board.height],
            captured_sheep: vec![None; board.width * board.height],
            time: 0,
        }
    }

    fn reachable_static_sheep(&mut self, max_dist: usize) -> usize {
        let mut pending = vec![self.board.dragon];
        let mut next = Vec::new();
        self.visited.fill(false);
        let mut reachable = 0;
        for _ in 0..=max_dist {
            for &(r, c) in &pending {
                let vis = &mut self.visited[r * self.board.width + c];
                if *vis {
                    continue;
                }
                *vis = true;

                if self.board.sheep[r * self.board.width + c] {
                    self.captured_sheep[r * self.board.width + c] = Some(0);
                    reachable += 1;
                }

                for (r1, c1) in [
                    (r >= 2 && c >= 1).then(|| (r - 2, c - 1)),
                    (r >= 2 && c + 1 < self.board.width).then(|| (r - 2, c + 1)),
                    (r >= 1 && c >= 2).then(|| (r - 1, c - 2)),
                    (r >= 1 && c + 2 < self.board.width).then(|| (r - 1, c + 2)),
                    (r + 1 < self.board.height && c >= 2).then(|| (r + 1, c - 2)),
                    (r + 1 < self.board.height && c + 2 < self.board.width).then(|| (r + 1, c + 2)),
                    (r + 2 < self.board.height && c >= 1).then(|| (r + 2, c - 1)),
                    (r + 2 < self.board.height && c + 1 < self.board.width).then(|| (r + 2, c + 1)),
                ]
                .into_iter()
                .flatten()
                {
                    if !self.visited[r1 * self.board.width + c1] {
                        next.push((r1, c1));
                    }
                }
            }
            (next, pending) = (pending, next);
            next.clear();
        }
        reachable
    }

    fn reachable_moving_sheep(&mut self, max_dist: usize) -> usize {
        let (r, c) = self.board.dragon;
        let mut pending = vec![
            (r - 2, c - 1),
            (r - 2, c + 1),
            (r - 1, c - 2),
            (r - 1, c + 2),
            (r + 1, c - 2),
            (r + 1, c + 2),
            (r + 2, c - 1),
            (r + 2, c + 1),
        ];
        let mut next = Vec::new();
        self.visited.fill(false);
        self.captured_sheep.fill(None);
        let mut reachable = 0;
        for time in 0..max_dist {
            self.time = time;
            for &(r, c) in &pending {
                let vis = &mut self.visited[r * self.board.width + c];
                if *vis {
                    continue;
                }
                *vis = true;

                for (r1, c1) in [
                    (r >= 2 && c >= 1).then(|| (r - 2, c - 1)),
                    (r >= 2 && c + 1 < self.board.width).then(|| (r - 2, c + 1)),
                    (r >= 1 && c >= 2).then(|| (r - 1, c - 2)),
                    (r >= 1 && c + 2 < self.board.width).then(|| (r - 1, c + 2)),
                    (r + 1 < self.board.height && c >= 2).then(|| (r + 1, c - 2)),
                    (r + 1 < self.board.height && c + 2 < self.board.width).then(|| (r + 1, c + 2)),
                    (r + 2 < self.board.height && c >= 1).then(|| (r + 2, c - 1)),
                    (r + 2 < self.board.height && c + 1 < self.board.width).then(|| (r + 2, c + 1)),
                ]
                .into_iter()
                .flatten()
                {
                    if !self.visited[r1 * self.board.width + c1] {
                        next.push((r1, c1));
                    }
                }
            }
            for r in 0..self.board.height {
                for c in 0..self.board.width {
                    for t in 0..=1 {
                        let time1 = time + t;
                        if (r + c + self.board.dragon.0 + self.board.dragon.1 + time) & 1 == 1
                            && r >= time1
                            && self.visited[r * self.board.width + c]
                            && self.board.sheep[(r - time1) * self.board.width + c]
                            && self.captured_sheep[(r - time1) * self.board.width + c].is_none()
                            && !self.board.blocked[r * self.board.width + c]
                        {
                            self.captured_sheep[(r - time1) * self.board.width + c] = Some(time);
                            reachable += 1;
                        }
                    }
                }
            }
            (next, pending) = (pending, next);
            next.clear();
        }
        reachable
    }
}

impl Display for QuantumDragon<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.board.height {
            writeln!(f)?;
            for c in 0..self.board.width {
                if self.visited[r * self.board.width + c]
                    && (r + c + self.board.dragon.0 + self.board.dragon.1 + self.time) & 1 == 1
                {
                    write!(f, "X")?;
                } else {
                    write!(f, ".")?;
                }
            }
            for time1 in self.time..=self.time + 1 {
                write!(f, "  ")?;
                for c in 0..self.board.width {
                    let sheep = r >= time1 && self.board.sheep[(r - time1) * self.board.width + c];
                    let captured_now = r >= time1
                        && self.captured_sheep[(r - time1) * self.board.width + c]
                            == Some(self.time);
                    let captured_before = r >= time1
                        && self.captured_sheep[(r - time1) * self.board.width + c].is_some()
                        && !captured_now;
                    let parity =
                        (r + c + self.board.dragon.0 + self.board.dragon.1 + self.time) & 1 == 1;
                    let blocked = self.board.blocked[r * self.board.width + c];
                    match (sheep && !captured_before, captured_now && parity, blocked) {
                        (true, _, true) => write!(f, "\x1b[1;32mS\x1b[0m")?,
                        (true, true, _) => write!(f, "\x1b[1;31mX\x1b[0m")?,
                        (true, _, _) => write!(f, "S")?,
                        _ => write!(f, ".")?,
                    }
                }
            }
            write!(f, "  ")?;
            let time1 = self.time + 1;
            for c in 0..self.board.width {
                let sheep = r >= time1 && self.board.sheep[(r - time1) * self.board.width + c];
                let captured =
                    r >= time1 && self.captured_sheep[(r - time1) * self.board.width + c].is_some();
                let blocked = self.board.blocked[r * self.board.width + c];
                match (sheep && !captured, blocked) {
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
    dragon: (usize, usize),
    sheep: Vec<Option<usize>>,
    safe: Vec<usize>,
}

type CacheKey = (bool, (usize, usize), Vec<Option<usize>>);

impl<'a> Game<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            dragon: board.dragon,
            sheep: (0..board.width)
                .map(|c| (0..board.height).find(|r| board.sheep[r * board.width + c]))
                .collect(),
            safe: (0..board.width)
                .map(|c| {
                    (1..=board.height)
                        .rev()
                        .find(|r| !board.blocked[(r - 1) * board.width + c])
                        .unwrap_or(99)
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

    fn dragon_moves(&mut self, seen: &mut HashMap<CacheKey, usize>) -> usize {
        let cache_key = self.cache_key(true);
        if let Some(&cached) = seen.get(&cache_key) {
            return cached;
        }
        let (r, c) = self.dragon;
        let mut count = 0;
        for (r1, c1) in [
            (r >= 1 && c >= 2).then(|| (r - 1, c - 2)),
            (r + 1 < self.board.height && c >= 2).then(|| (r + 1, c - 2)),
            (r >= 2 && c >= 1).then(|| (r - 2, c - 1)),
            (r + 2 < self.board.height && c >= 1).then(|| (r + 2, c - 1)),
            (r >= 2 && c + 1 < self.board.width).then(|| (r - 2, c + 1)),
            (r + 2 < self.board.height && c + 1 < self.board.width).then(|| (r + 2, c + 1)),
            (r >= 1 && c + 2 < self.board.width).then(|| (r - 1, c + 2)),
            (r + 1 < self.board.height && c + 2 < self.board.width).then(|| (r + 1, c + 2)),
        ]
        .into_iter()
        .flatten()
        {
            self.dragon = (r1, c1);
            if !self.board.blocked[r1 * self.board.width + c1] && self.sheep[c1] == Some(r1) {
                self.sheep[c1] = None;
                count += self.sheep_moves(seen);
                self.sheep[c1] = Some(r1);
            } else {
                count += self.sheep_moves(seen);
            }
        }
        self.dragon = (r, c);
        seen.insert(cache_key, count);
        count
    }

    fn sheep_moves(&mut self, seen: &mut HashMap<CacheKey, usize>) -> usize {
        let cache_key = self.cache_key(false);
        if let Some(&cached) = seen.get(&cache_key) {
            return cached;
        }
        if self.sheep.iter().all(Option::is_none) {
            return 1;
        }
        let mut count = 0;
        let mut any_move = false;
        for c in 0..self.board.width {
            let Some(r) = self.sheep[c] else { continue };
            let r1 = r + 1;
            if (r1, c) == self.dragon && !self.board.blocked[r1 * self.board.width + c] {
                continue;
            }
            // From the sheeps perspective, the only illegal move is to move into the dragon.
            // So for the purpuse of allowing double moves, only count this one, even every other
            // move leads to sheeps escaping.
            any_move = true;
            if r1 == self.board.height || r1 >= self.safe[c] {
                continue;
            }
            self.sheep[c] = Some(r1);
            count += self.dragon_moves(seen);
            self.sheep[c] = Some(r);
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
                if self.dragon == (r, c) {
                    write!(f, "D")?;
                } else if self.sheep[c] == Some(r) {
                    if self.board.blocked[r * self.board.width + c] {
                        write!(f, "\x1b[1;32m#\x1b[0m")?;
                    } else {
                        write!(f, "s")?;
                    }
                } else if self.board.blocked[r * self.board.width + c] {
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
        QuantumDragon::new(input).reachable_static_sheep(4)
    }

    fn part_2(input: &Self::Input) -> usize {
        QuantumDragon::new(input).reachable_moving_sheep(20)
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
        let result = QuantumDragon::new(&board).reachable_static_sheep(3);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_part_2() {
        let board = Day10::parse(P2_EXAMPLE).unwrap();
        let result = QuantumDragon::new(&board).reachable_moving_sheep(3);
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
