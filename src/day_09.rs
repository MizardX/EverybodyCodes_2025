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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nucleobase {
    C,
    G,
    A,
    T,
}

impl TryFrom<u8> for Nucleobase {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'C' => Self::C,
            b'G' => Self::G,
            b'A' => Self::A,
            b'T' => Self::T,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

pub struct ScaleDNA {
    id: usize,
    dna: Vec<Nucleobase>,
}

impl FromStr for ScaleDNA {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, dna) = s.split_once(':').ok_or(ParseError::SyntaxError)?;
        let id = id.parse()?;
        let dna = dna
            .bytes()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { id, dna })
    }
}

struct UFNode {
    parent: usize,
    size: usize,
    sum: usize,
}

struct UnionFind {
    nodes: Vec<UFNode>,
}

impl UnionFind {
    fn new(input: &[ScaleDNA]) -> Self {
        let nodes = input
            .iter()
            .enumerate()
            .map(|(ix, scale)| UFNode {
                parent: ix,
                size: 1,
                sum: scale.id,
            })
            .collect();
        Self { nodes }
    }

    fn find(&mut self, mut index: usize) -> usize {
        let mut parent = self.nodes[index].parent;
        while index != parent {
            let grand_parent = self.nodes[parent].parent;
            self.nodes[index].parent = grand_parent;
            index = grand_parent;
            parent = self.nodes[index].parent;
        }
        index
    }

    fn union(&mut self, mut index1: usize, mut index2: usize) -> bool {
        index1 = self.find(index1);
        index2 = self.find(index2);
        if index1 == index2 {
            return false;
        }
        if self.nodes[index1].size < self.nodes[index2].size {
            (index1, index2) = (index2, index1);
        }
        self.nodes[index2].parent = index1;
        self.nodes[index1].size += self.nodes[index2].size;
        self.nodes[index1].sum += self.nodes[index2].sum;
        true
    }

    fn is_root(&self, index: usize) -> bool {
        self.nodes[index].parent == index
    }

    fn size(&self, index: usize) -> Option<usize> {
        self.is_root(index).then(|| self.nodes[index].size)
    }

    fn sum(&self, index: usize) -> Option<usize> {
        self.is_root(index).then(|| self.nodes[index].sum)
    }
}

pub struct Day09;

impl crate::Day for Day09 {
    type Input = Vec<ScaleDNA>;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part_1(input: &Self::Input) -> usize {
        for (child_ix, child) in input.iter().enumerate() {
            let parent1 = &input[(child_ix + 1) % 3];
            let parent2 = &input[(child_ix + 2) % 3];
            if let Some(similarity) = degree_of_similarity(child, parent1, parent2) {
                return similarity;
            }
        }
        0
    }

    fn part_2(input: &Self::Input) -> usize {
        let mut scores = 0;
        'next_child: for (child_ix, child) in input.iter().enumerate() {
            for (parent1_ix, parent1) in input.iter().enumerate() {
                if parent1_ix == child_ix {
                    continue;
                }
                for (parent2_ix, parent2) in input[..parent1_ix].iter().enumerate() {
                    if parent2_ix == child_ix {
                        continue;
                    }
                    if let Some(similarity) = degree_of_similarity(child, parent1, parent2) {
                        scores += similarity;
                        continue 'next_child;
                    }
                }
            }
        }
        scores
    }

    fn part_3(input: &Self::Input) -> usize {
        let mut uf = UnionFind::new(input);
        'child: for (child_ix, child) in input.iter().enumerate() {
            for (parent1_ix, parent1) in input.iter().enumerate() {
                if parent1_ix == child_ix {
                    continue;
                }
                for (parent2_ix, parent2) in input[..parent1_ix].iter().enumerate() {
                    if parent2_ix == child_ix {
                        continue;
                    }
                    if degree_of_similarity(child, parent1, parent2).is_some() {
                        uf.union(parent1_ix, child_ix);
                        uf.union(parent2_ix, child_ix);
                        continue 'child;
                    }
                }
            }
        }
        let mut max_size = 0;
        let mut max_size_sum = 0;
        for ix in 0..input.len() {
            if let Some(size) = uf.size(ix)
                && size > max_size
            {
                max_size = size;
                max_size_sum = uf.sum(ix).unwrap();
            }
        }
        max_size_sum
    }
}

fn degree_of_similarity(child: &ScaleDNA, parent1: &ScaleDNA, parent2: &ScaleDNA) -> Option<usize> {
    let mut score1 = 0;
    let mut score2 = 0;
    for ((p1, p2), c1) in parent1.dna.iter().zip(&parent2.dna).zip(&child.dna) {
        match (c1 == p1, c1 == p2) {
            (true, true) => {
                score1 += 1;
                score2 += 1;
            }
            (true, false) => score1 += 1,
            (false, true) => score2 += 1,
            (false, false) => return None,
        }
    }
    Some(score1 * score2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;
    use test_case::test_case;

    const EXAMPLE1: &str = "\
        1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG\n\
        2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT\n\
        3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG\
    ";

    const EXAMPLE2: &str = "\
        1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC\n\
        2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC\n\
        3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG\n\
        4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA\n\
        5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA\n\
        6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA\n\
        7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG\
    ";

    const EXAMPLE3: &str = "\
        1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC\n\
        2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC\n\
        3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG\n\
        4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA\n\
        5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA\n\
        6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA\n\
        7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG\n\
        8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG\
    ";

    #[test_case(EXAMPLE1 => 414)]
    fn test_part_1(input: &str) -> usize {
        let scales = Day09::parse(input).unwrap();
        Day09::part_1(&scales)
    }

    #[test_case(EXAMPLE2 => 1245)]
    fn test_part_2(input: &str) -> usize {
        let scales = Day09::parse(input).unwrap();
        Day09::part_2(&scales)
    }

    #[test_case(EXAMPLE2 => 12)]
    #[test_case(EXAMPLE3 => 36)]
    fn test_part_3(input: &str) -> usize {
        let scales = Day09::parse(input).unwrap();
        Day09::part_3(&scales)
    }
}
