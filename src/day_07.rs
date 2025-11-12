use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    before: u8,
    after: u64, // bitfield
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" > ").ok_or(ParseError::SyntaxError)?;
        let &[before @ (b'A'..=b'Z' | b'a'..=b'z')] = left.as_bytes() else {
            return Err(ParseError::SyntaxError);
        };
        let mut after = 0;
        for right in right.split(',') {
            let &[right @ (b'A'..=b'Z' | b'a'..=b'z')] = right.as_bytes() else {
                return Err(ParseError::SyntaxError);
            };
            after |= 1 << (right - b'A');
        }
        Ok(Self { before, after })
    }
}

#[derive(Clone)]
pub struct RuleSet {
    rules: [u64; 58],
}

impl Debug for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut m = f.debug_map();
        for ch in (b'A'..=b'Z').chain(b'a'..=b'z') {
            let mask = self.rules[(ch - b'A') as usize];
            if mask == 0 {
                continue;
            }
            m.entry(
                &(ch as char),
                &(b'A'..=b'Z')
                    .chain(b'a'..=b'z')
                    .filter_map(|ch1| (mask & (1 << (ch1 - b'A')) != 0).then_some(ch1 as char))
                    .collect::<Vec<_>>(),
            );
        }
        m.finish()
    }
}

impl RuleSet {
    const fn is_valid(&self, before: u8, after: u8) -> bool {
        (self.rules[(before - b'A') as usize] & (1 << (after - b'A'))) != 0
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "trailing_zeros never exceeds 64"
    )]
    const fn first_valid(&self, before: u8) -> Option<u8> {
        let bits = self.rules[(before - b'A') as usize];
        if bits != 0 {
            Some(bits.trailing_zeros() as u8 + b'A')
        } else {
            None
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "trailing_zeros never exceeds 64"
    )]
    const fn next_valid(&self, before: u8, after: u8) -> Option<u8> {
        let mask = !(!0 << (after - b'A' + 1));
        let bits = self.rules[(before - b'A') as usize] & !mask;
        if bits != 0 {
            let next = bits.trailing_zeros() as u8 + b'A';
            Some(next)
        } else {
            None
        }
    }
}

impl FromIterator<Rule> for RuleSet {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        let mut rules = [0; 58];
        for rule in iter {
            rules[(rule.before - b'A') as usize] |= rule.after;
        }
        Self { rules }
    }
}

pub struct Input {
    names: Vec<String>,
    rules: RuleSet,
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let names = lines
            .next()
            .ok_or(ParseError::SyntaxError)?
            .split(',')
            .map(ToString::to_string)
            .collect();
        if lines.next() != Some("") {
            return Err(ParseError::SyntaxError);
        }
        let rules = lines.map(str::parse).collect::<Result<_, ParseError>>()?;
        Ok(Self { names, rules })
    }
}

pub struct Day07;

impl crate::Day for Day07 {
    type Input = Input;

    type ParseError = ParseError;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.parse()
    }

    fn part_1(input: &Self::Input) -> String {
        input
            .names
            .iter()
            .find(|name| is_valid(name, input))
            .map_or_else(Default::default, Clone::clone)
    }

    fn part_2(input: &Self::Input) -> usize {
        input
            .names
            .iter()
            .zip(1..)
            .filter_map(|(name, id)| is_valid(name, input).then_some(id))
            .sum()
    }

    fn part_3(input: &Self::Input) -> usize {
        let mut count = 0;
        let mut names = input.names.iter().map(String::as_str).collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup_by(|a, b| a.starts_with(*b));
        let mut cache = HashMap::new();
        for name in &names {
            if name.len() > 11 {
                continue;
            }
            if is_valid(name, input) {
                count += count_possible_continuations(
                    name.bytes().last().unwrap(),
                    7_usize.saturating_sub(name.len()),
                    11 - name.len(),
                    input,
                    &mut cache,
                );
            }
        }
        count
    }
}

fn is_valid(name: &str, input: &Input) -> bool {
    let mut prev = name.as_bytes()[0];
    for ch in name.bytes().skip(1) {
        if input.rules.is_valid(prev, ch) {
            prev = ch;
        } else {
            return false;
        }
    }
    true
}

fn count_possible_continuations(
    prev: u8,
    min_len: usize,
    max_len: usize,
    input: &Input,
    cache: &mut HashMap<(usize, u8), usize>,
) -> usize {
    let mut sum = 0;
    if min_len == 0 {
        sum += 1;
    }
    if max_len == 0 {
        return sum;
    }
    if let Some(&old) = cache.get(&(max_len, prev)) {
        return old;
    }
    if let Some(mut ch) = input.rules.first_valid(prev) {
        sum +=
            count_possible_continuations(ch, min_len.saturating_sub(1), max_len - 1, input, cache);
        while let Some(ch1) = input.rules.next_valid(prev, ch) {
            sum += count_possible_continuations(
                ch1,
                min_len.saturating_sub(1),
                max_len - 1,
                input,
                cache,
            );
            ch = ch1;
        }
    }
    cache.insert((max_len, prev), sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    use test_case::test_case;

    const EXAMPLE1: &str = "\
        Oronris,Urakris,Oroneth,Uraketh\n\
        \n\
        r > a,i,o\n\
        i > p,w\n\
        n > e,r\n\
        o > n,m\n\
        k > f,r\n\
        a > k\n\
        U > r\n\
        e > t\n\
        O > r\n\
        t > h\
    ";
    const EXAMPLE2: &str = "\
        Xanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax\n\
        \n\
        r > v,e,a,g,y\n\
        a > e,v,x,r\n\
        e > r,x,v,t\n\
        h > a,e,v\n\
        g > r,y\n\
        y > p,t\n\
        i > v,r\n\
        K > h\n\
        v > e\n\
        B > r\n\
        t > h\n\
        N > e\n\
        p > h\n\
        H > e\n\
        l > t\n\
        z > e\n\
        X > a\n\
        n > v\n\
        x > z\n\
        T > i\n\
    ";

    const EXAMPLE3: &str = "\
        Xaryt\n\
        \n\
        X > a,o\n\
        a > r,t\n\
        r > y,e,a\n\
        h > a,e,v\n\
        t > h\n\
        v > e\n\
        y > p,t\
    ";

    const EXAMPLE4: &str = "\
        Khara,Xaryt,Noxer,Kharax\n\
        \n\
        r > v,e,a,g,y\n\
        a > e,v,x,r,g\n\
        e > r,x,v,t\n\
        h > a,e,v\n\
        g > r,y\n\
        y > p,t\n\
        i > v,r\n\
        K > h\n\
        v > e\n\
        B > r\n\
        t > h\n\
        N > e\n\
        p > h\n\
        H > e\n\
        l > t\n\
        z > e\n\
        X > a\n\
        n > v\n\
        x > z\n\
        T > i\
    ";

    #[test]
    fn test_part_1() {
        let input = Day07::parse(EXAMPLE1).unwrap();
        let result = Day07::part_1(&input);
        assert_eq!(result, "Oroneth");
    }

    #[test]
    fn test_part_2() {
        let input = Day07::parse(EXAMPLE2).unwrap();
        let result = Day07::part_2(&input);
        assert_eq!(result, 23);
    }

    #[test_case(EXAMPLE3 => 25)]
    #[test_case(EXAMPLE4 => 1154)]
    fn test_part_3(input: &str) -> usize {
        let input = Day07::parse(input).unwrap();
        Day07::part_3(&input)
    }
}
