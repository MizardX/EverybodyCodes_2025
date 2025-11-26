use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Volcano,
    Start,
    Cell(u8),
}

impl TryFrom<u8> for Tile {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'@' => Self::Volcano,
            b'S' => Self::Start,
            b'0'..=b'9' => Self::Cell(value - b'0'),
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

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<u8>,
    volcano: Option<(usize, usize)>,
    start: Option<(usize, usize)>,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let height = lines.clone().count();
        let width = lines.clone().next().ok_or(ParseError::SyntaxError)?.len();
        let mut volcano = None;
        let mut start = None;
        let mut data = Vec::with_capacity(width * height);
        for (r, row) in lines.enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                data.push(match Tile::try_from(ch)? {
                    Tile::Volcano => {
                        volcano = Some((r, c));
                        0
                    }
                    Tile::Start => {
                        start = Some((r, c));
                        0
                    }
                    Tile::Cell(val) => val,
                });
            }
        }
        let grid = Grid::new(data, width, height);
        Ok(Self {
            grid,
            volcano,
            start,
        })
    }
}

fn sum_within_radius(grid: &Grid<u8>, volcano: (usize, usize), radius: usize) -> u64 {
    (0..grid.height)
        .flat_map(|r| (0..grid.width).map(move |c| (r, c)))
        .filter(|&pos| {
            let dr = pos.0.abs_diff(volcano.0);
            let dc = pos.1.abs_diff(volcano.1);
            dr * dr + dc * dc <= radius * radius
        })
        .map(|pos| u64::from(grid[pos]))
        .sum()
}

fn sum_by_distance(grid: &Grid<u8>, volcano: (usize, usize)) -> Vec<u64> {
    let mut sum_by_dist = vec![0; grid.width + grid.height];
    let mut max_dist = usize::MAX;
    for r in 0..grid.height {
        for c in 0..grid.width {
            let dr = r.abs_diff(volcano.0);
            let dc = c.abs_diff(volcano.1);
            let mut dist = (dr * dr + dc * dc).isqrt();
            if dr * dr + dc * dc != dist * dist {
                dist += 1; // round up
            }
            if r == 0 || r == grid.height - 1 || c == 0 || c == grid.width - 1 {
                max_dist = max_dist.min(dist);
            }
            sum_by_dist[dist] += u64::from(grid[(r, c)]);
        }
    }
    sum_by_dist.truncate(max_dist + 1);
    sum_by_dist
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum State {
    Start = 0,
    Left = 1,
    Right = 2,
}

fn perimiter_sum(grid: &Grid<u8>, volcano: (usize, usize), start: (usize, usize)) -> Option<u64> {
    let mut pending = BinaryHeap::new();
    let mut visited = Grid::<[u64; 3]>::new(
        vec![[u64::MAX; 3]; grid.data.len()],
        grid.width,
        grid.height,
    );
    for radius in 0.. {
        let max_dist = (radius + 1) * 30 - 1;
        visited.data.fill([u64::MAX; 3]);
        pending.clear();
        pending.push((Reverse(0), State::Start, start));
        visited[start][0] = 0;
        while let Some((Reverse(dist), state, (r, c))) = pending.pop() {
            let new_state = match state {
                State::Start if r == volcano.0 && c < volcano.1 => State::Left,
                State::Start if r == volcano.0 && c > volcano.1 => State::Right,
                State::Start if r > volcano.0 => continue,
                State::Left | State::Right if r < volcano.0 => continue,
                State::Left | State::Right if r > volcano.0 && c == volcano.1 => {
                    continue;
                }
                copy => copy,
            };
            if dist >= max_dist {
                continue;
            }
            for (r1, c1) in [
                r.checked_sub(1).map(|r1| (r1, c)),
                r.checked_add(1)
                    .filter(|&r1| r1 < grid.height)
                    .map(|r1| (r1, c)),
                c.checked_sub(1).map(|c1| (r, c1)),
                c.checked_add(1)
                    .filter(|&c1| c1 < grid.width)
                    .map(|c1| (r, c1)),
            ]
            .into_iter()
            .flatten()
            {
                let dr = u64::try_from(r1.abs_diff(volcano.0)).unwrap();
                let dc = u64::try_from(c1.abs_diff(volcano.1)).unwrap();
                if dr * dr + dc * dc <= radius * radius {
                    continue;
                }
                let new_dist = match grid[(r1, c1)] {
                    0 => continue,
                    val => dist + u64::from(val),
                };
                if visited[(r1, c1)][new_state as usize] <= new_dist {
                    continue;
                }
                visited[(r1, c1)][new_state as usize] = new_dist;
                pending.push((Reverse(new_dist), new_state, (r1, c1)));
            }
        }
        if let Some(dist) = (volcano.0 + 1..grid.height)
            .filter_map(|row| {
                let [_, l, r] = visited[(row, volcano.1)];
                if l < u64::MAX && r < u64::MAX {
                    match grid[(row, volcano.1)] {
                        val @ 1.. if l + r - u64::from(val) <= max_dist => {
                            Some(l + r - u64::from(val))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .min()
        {
            return Some(dist * radius);
        }
    }
    None
}

pub struct Day17;

impl crate::Day for Day17 {
    type Input = Input;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> u64 {
        let volcano = input.volcano.unwrap();
        sum_within_radius(&input.grid, volcano, 10)
    }

    fn part_2(input: &Self::Input) -> u64 {
        let volcano = input.volcano.unwrap();
        let sum_by_dist = sum_by_distance(&input.grid, volcano);
        sum_by_dist
            .iter()
            .enumerate()
            .max_by_key(|&(_, &val)| val)
            .map(|(r, &val)| u64::try_from(r).unwrap() * val)
            .unwrap()
    }

    fn part_3(input: &Self::Input) -> u64 {
        let start = input.start.unwrap();
        let volcano = input.volcano.unwrap();
        perimiter_sum(&input.grid, volcano, start).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    use test_case::test_case;

    const EXAMPLE1: &str = "\
        189482189843433862719\n\
        279415473483436249988\n\
        432746714658787816631\n\
        428219317375373724944\n\
        938163982835287292238\n\
        627369424372196193484\n\
        539825864246487765271\n\
        517475755641128575965\n\
        685934212385479112825\n\
        815992793826881115341\n\
        1737798467@7983146242\n\
        867597735651751839244\n\
        868364647534879928345\n\
        519348954366296559425\n\
        134425275832833829382\n\
        764324337429656245499\n\
        654662236199275446914\n\
        317179356373398118618\n\
        542673939694417586329\n\
        987342622289291613318\n\
        971977649141188759131\
    ";

    const EXAMPLE2: &str = "\
        4547488458944\n\
        9786999467759\n\
        6969499575989\n\
        7775645848998\n\
        6659696497857\n\
        5569777444746\n\
        968586@767979\n\
        6476956899989\n\
        5659745697598\n\
        6874989897744\n\
        6479994574886\n\
        6694118785585\n\
        9568991647449\
    ";

    const EXAMPLE3A: &str = "\
        2645233S5466644\n\
        634566343252465\n\
        353336645243246\n\
        233343552544555\n\
        225243326235365\n\
        536334634462246\n\
        666344656233244\n\
        6426432@2366453\n\
        364346442652235\n\
        253652463426433\n\
        426666225623563\n\
        555462553462364\n\
        346225464436334\n\
        643362324542432\n\
        463332353552464\
    ";

    const EXAMPLE3B: &str = "\
        545233443422255434324\n\
        5222533434S2322342222\n\
        523444354223232542432\n\
        553522225435232255242\n\
        232343243532432452524\n\
        245245322252324442542\n\
        252533232225244224355\n\
        523533554454232553332\n\
        522332223232242523223\n\
        524523432425432244432\n\
        3532242243@4323422334\n\
        542524223994422443222\n\
        252343244322522222332\n\
        253355425454255523242\n\
        344324325233443552555\n\
        423523225325255345522\n\
        244333345244325322335\n\
        242244352245522323422\n\
        443332352222535334325\n\
        323532222353523253542\n\
        553545434425235223552\
    ";

    const EXAMPLE3C: &str = "\
        5441525241225111112253553251553\n\
        133522122534119S911411222155114\n\
        3445445533355599933443455544333\n\
        3345333555434334535435433335533\n\
        5353333345335554434535533555354\n\
        3533533435355443543433453355553\n\
        3553353435335554334453355435433\n\
        5435355533533355533535335345335\n\
        4353545353545354555534334453353\n\
        4454543553533544443353355553453\n\
        5334554534533355333355543533454\n\
        4433333345445354553533554555533\n\
        5554454343455334355445533453453\n\
        4435554534445553335434455334353\n\
        3533435453433535345355533545555\n\
        534433533533535@353533355553345\n\
        4453545555435334544453344455554\n\
        4353333535535354535353353535355\n\
        4345444453554554535355345343354\n\
        3534544535533355333333445433555\n\
        3535333335335334333534553543535\n\
        5433355333553344355555344553435\n\
        5355535355535334555435534555344\n\
        3355433335553553535334544544333\n\
        3554333535553335343555345553535\n\
        3554433545353554334554345343343\n\
        5533353435533535333355343333555\n\
        5355555353355553535354333535355\n\
        4344534353535455333455353335333\n\
        5444333535533453535335454535553\n\
        3534343355355355553543545553345\
    ";

    #[test]
    fn test_part_1() {
        let input = Day17::parse(EXAMPLE1).unwrap();
        let result = Day17::part_1(&input);
        assert_eq!(result, 1573);
    }

    #[test]
    fn test_part_2() {
        let input = Day17::parse(EXAMPLE2).unwrap();
        let result = Day17::part_2(&input);
        assert_eq!(result, 1090);
    }

    #[test_case(EXAMPLE3A => 592)]
    #[test_case(EXAMPLE3B => 330)]
    #[test_case(EXAMPLE3C => 3180)]
    fn test_part_3(input: &str) -> u64 {
        let input = Day17::parse(input).unwrap();
        Day17::part_3(&input)
    }
}
