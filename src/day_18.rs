use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plant {
    id: usize,
    thickness: u64,
    branches: Vec<Branch>,
}

impl Plant {
    const fn is_free(&self) -> bool {
        self.branches.is_empty()
    }
}

impl FromStr for Plant {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first = lines.next().ok_or(ParseError::SyntaxError)?;
        let rest = first
            .strip_prefix("Plant ")
            .ok_or(ParseError::SyntaxError)?;
        let (id, rest) = rest
            .split_once(" with thickness ")
            .ok_or(ParseError::SyntaxError)?;
        let id: usize = id.parse()?;
        let thickness: u64 = rest
            .strip_suffix(":")
            .ok_or(ParseError::SyntaxError)?
            .parse()?;
        if lines.clone().next() == Some("- free branch with thickness 1") {
            Ok(Self {
                id,
                thickness,
                branches: Vec::new(),
            })
        } else {
            let branches = lines.map(str::parse).collect::<Result<_, _>>()?;
            Ok(Self {
                id,
                thickness,
                branches,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Branch {
    thickness: i64,
    connected_to: usize,
}

impl FromStr for Branch {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("- branch to Plant ") {
            let (plant, thickness) = rest
                .split_once(" with thickness ")
                .ok_or(ParseError::SyntaxError)?;
            Ok(Self {
                thickness: thickness.parse()?,
                connected_to: plant.parse()?,
            })
        } else {
            Err(ParseError::SyntaxError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    plants: Vec<Plant>,
    test_cases: Vec<u128>, // bitmasks
}

impl Input {
    fn final_plant_energy(&self, configuration: u128, energy: &mut Vec<i64>) -> i64 {
        energy.clear();
        for plant in &self.plants {
            if plant.is_free() {
                energy.push(((configuration >> (plant.id - 1)) & 1) as i64);
            } else {
                let mut incoming = 0;
                for &branch in &plant.branches {
                    incoming += energy[branch.connected_to - 1] * branch.thickness;
                }
                energy.push(if incoming >= plant.thickness.cast_signed() {
                    incoming
                } else {
                    0
                });
            }
        }
        energy.last().copied().unwrap()
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n\n");
        let plants = parts
            .next()
            .ok_or(ParseError::SyntaxError)?
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        let test_cases = parts
            .next()
            .iter()
            .flat_map(|cases| {
                cases.lines().map(|line| {
                    line.split(' ')
                        .enumerate()
                        .try_fold(0, |mask, (ix, val)| Ok(mask | val.parse::<u128>()? << ix))
                })
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        Ok(Self { plants, test_cases })
    }
}

pub struct Day18;

impl crate::Day for Day18 {
    type Input = Input;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> i64 {
        input.final_plant_energy(u128::MAX, &mut Vec::new())
    }

    fn part_2(input: &Self::Input) -> i64 {
        let mut energy = Vec::new();
        input
            .test_cases
            .iter()
            .map(|&test_case| input.final_plant_energy(test_case, &mut energy))
            .sum()
    }

    fn part_3(input: &Self::Input) -> i64 {
        let mut energy = Vec::new();
        let mut max_configuration = 0_u128;
        let num_free = input.plants.iter().filter(|p| p.is_free()).count();
        if num_free < 9 {
            max_configuration = (0..(1 << num_free))
                .max_by_key(|&configuration| input.final_plant_energy(configuration, &mut energy))
                .unwrap();
        } else {
            // Exploit that the input layer nodes always has all positive or all negative edges.
            // Also, the only layer with negative weights is the input layer.
            // We can choose the maximal configuration by just looking at the sign of the branch thicknesses.
            for p in &input.plants {
                if p.branches
                    .first()
                    .is_some_and(|b| input.plants[b.connected_to - 1].is_free())
                {
                    for &b in &p.branches {
                        if b.thickness > 0 {
                            max_configuration |= 1 << (b.connected_to - 1);
                        }
                    }
                }
            }
        }
        let max_energy = input.final_plant_energy(max_configuration, &mut energy);
        input
            .test_cases
            .iter()
            .filter_map(|&test_case| {
                let res = input.final_plant_energy(test_case, &mut energy);
                if res > 0 {
                    Some(max_energy - res)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    const EXAMPLE1: &str = "\
        Plant 1 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 2 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 3 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 4 with thickness 17:\n\
        - branch to Plant 1 with thickness 15\n\
        - branch to Plant 2 with thickness 3\n\
        \n\
        Plant 5 with thickness 24:\n\
        - branch to Plant 2 with thickness 11\n\
        - branch to Plant 3 with thickness 13\n\
        \n\
        Plant 6 with thickness 15:\n\
        - branch to Plant 3 with thickness 14\n\
        \n\
        Plant 7 with thickness 10:\n\
        - branch to Plant 4 with thickness 15\n\
        - branch to Plant 5 with thickness 21\n\
        - branch to Plant 6 with thickness 34\
    ";

    #[test]
    fn test_part_1() {
        let input = Day18::parse(EXAMPLE1).unwrap();
        let result = Day18::part_1(&input);
        assert_eq!(result, 774);
    }

    const EXAMPLE2: &str = "\
        Plant 1 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 2 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 3 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 4 with thickness 10:\n\
        - branch to Plant 1 with thickness -25\n\
        - branch to Plant 2 with thickness 17\n\
        - branch to Plant 3 with thickness 12\n\
        \n\
        Plant 5 with thickness 14:\n\
        - branch to Plant 1 with thickness 14\n\
        - branch to Plant 2 with thickness -26\n\
        - branch to Plant 3 with thickness 15\n\
        \n\
        Plant 6 with thickness 150:\n\
        - branch to Plant 4 with thickness 5\n\
        - branch to Plant 5 with thickness 6\n\
        \n\
        \n\
        1 0 1\n\
        0 0 1\n\
        0 1 1\
    ";

    #[test]
    fn test_part_2() {
        let input = Day18::parse(EXAMPLE2).unwrap();
        let result = Day18::part_2(&input);
        assert_eq!(result, 324);
    }

    const EXAMPLE3: &str = "\
        Plant 1 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 2 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 3 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 4 with thickness 1:\n\
        - free branch with thickness 1\n\
        \n\
        Plant 5 with thickness 8:\n\
        - branch to Plant 1 with thickness -8\n\
        - branch to Plant 2 with thickness 11\n\
        - branch to Plant 3 with thickness 13\n\
        - branch to Plant 4 with thickness -7\n\
        \n\
        Plant 6 with thickness 7:\n\
        - branch to Plant 1 with thickness 14\n\
        - branch to Plant 2 with thickness -9\n\
        - branch to Plant 3 with thickness 12\n\
        - branch to Plant 4 with thickness 9\n\
        \n\
        Plant 7 with thickness 23:\n\
        - branch to Plant 5 with thickness 17\n\
        - branch to Plant 6 with thickness 18\n\
        \n\
        \n\
        0 1 0 0\n\
        0 1 0 1\n\
        0 1 1 1\n\
        1 1 0 1\
    ";

    #[test]
    fn test_part_3() {
        let input = Day18::parse(EXAMPLE3).unwrap();
        let result = Day18::part_3(&input);
        assert_eq!(result, 946);
    }
}
