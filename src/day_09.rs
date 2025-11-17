use std::cmp::Reverse;
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
#[repr(u8)]
pub enum Nucleobase {
    C = 1 << 0,
    G = 1 << 1,
    A = 1 << 2,
    T = 1 << 3,
}

impl From<Nucleobase> for u128 {
    fn from(value: Nucleobase) -> Self {
        value as Self
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScaleDNA {
    id: usize,
    mask: [u128; 4],
}

impl ScaleDNA {
    fn except(&self, other: &Self) -> Self {
        Self {
            id: other.id,
            mask: [0, 1, 2, 3].map(|ix| self.mask[ix] & !other.mask[ix]),
        }
    }

    fn intersect(&self, other: &Self) -> Self {
        Self {
            id: other.id,
            mask: [0, 1, 2, 3].map(|ix| self.mask[ix] & other.mask[ix]),
        }
    }

    fn count_ones(&self) -> u32 {
        self.mask.iter().map(|bits| bits.count_ones()).sum()
    }

    fn is_zero(&self) -> bool {
        self.mask == [0; 4]
    }

    fn degree_of_similarity(&self, parent1: &Self, parent2: &Self) -> Option<u32> {
        self.except(parent1).except(parent2).is_zero().then(|| {
            let score1 = self.intersect(parent1).count_ones();
            let score2 = self.intersect(parent2).count_ones();
            score1 * score2
        })
    }
}

impl FromStr for ScaleDNA {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, dna) = s.split_once(':').ok_or(ParseError::SyntaxError)?;
        let id = id.parse()?;
        let mut mask = [0_u128; 4];
        for (ix, nucl) in dna.bytes().enumerate() {
            let nucl: Nucleobase = nucl.try_into()?;
            mask[ix >> 5] |= (nucl as u128) << ((ix & 0x1f) << 2);
        }
        Ok(Self { id, mask })
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
        let mut nodes = input
            .iter()
            .map(|scale| UFNode {
                parent: scale.id - 1,
                size: 1,
                sum: scale.id,
            })
            .collect::<Vec<_>>();
        nodes.sort_unstable_by_key(|n| n.parent);
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

    fn part_1(input: &Self::Input) -> u32 {
        for (child_ix, child) in input.iter().enumerate() {
            let parent1 = &input[(child_ix + 1) % 3];
            let parent2 = &input[(child_ix + 2) % 3];
            if let Some(similarity) = child.degree_of_similarity(parent1, parent2) {
                return similarity;
            }
        }
        0
    }

    fn part_2(input: &Self::Input) -> u32 {
        let mut scores = 0;
        let mut ordered = input.clone();
        'next_child: for child in input {
            let top_3 = ordered
                .select_nth_unstable_by_key(4, |p| Reverse(child.intersect(p).count_ones()))
                .0;
            for (ix, p1) in top_3.iter().enumerate() {
                if p1.id == child.id {
                    continue;
                }
                for p2 in &top_3[..ix] {
                    if p2.id == child.id {
                        continue;
                    }
                    if let Some(score) = child.degree_of_similarity(p1, p2) {
                        scores += score;
                        continue 'next_child;
                    }
                }
            }
        }
        scores
    }

    fn part_3(input: &Self::Input) -> usize {
        let mut uf = UnionFind::new(input);
        let mut ordered = input.clone();
        'child: for child in input {
            let top_n = ordered
                .select_nth_unstable_by_key(7, |p| Reverse(child.intersect(p).count_ones()))
                .0;
            for (ix, parent1) in top_n.iter().enumerate() {
                if parent1.id == child.id {
                    continue;
                }
                for parent2 in &top_n[..ix] {
                    if parent2.id == child.id {
                        continue;
                    }
                    if child.degree_of_similarity(parent1, parent2).is_some() {
                        uf.union(parent1.id - 1, child.id - 1);
                        uf.union(parent2.id - 1, child.id - 1);
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
    fn test_part_1(input: &str) -> u32 {
        let scales = Day09::parse(input).unwrap();
        Day09::part_1(&scales)
    }

    #[test_case(EXAMPLE2 => 1245)]
    fn test_part_2(input: &str) -> u32 {
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
