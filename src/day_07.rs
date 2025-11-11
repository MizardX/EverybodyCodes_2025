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
        let &[before] = left.as_bytes() else {
            return Err(ParseError::SyntaxError);
        };
        let mut after = 0;
        for right in right.split(',') {
            let &[right] = right.as_bytes() else {
                return Err(ParseError::SyntaxError);
            };
            after |= 1 << (right - b'A');
        }
        Ok(Self { before, after })
    }
}

pub struct Input {
    names: Vec<String>,
    rules: Vec<Rule>,
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
        'names: for name in &input.names {
            let mut prev = name.as_bytes()[0];
            for ch in name.bytes().skip(1) {
                let Some(valid) = input
                    .rules
                    .iter()
                    .find_map(|r| (r.before == prev).then_some(r.after))
                else {
                    continue 'names;
                };
                if valid & (1 << (ch - b'A')) != 0 {
                    prev = ch;
                } else {
                    continue 'names;
                }
            }
            return name.clone();
        }
        String::new()
    }

    fn part_2(input: &Self::Input) -> usize {
        let mut sum = 0;
        'names: for (name, index) in input.names.iter().zip(1..) {
            let mut prev = name.as_bytes()[0];
            for ch in name.bytes().skip(1) {
                let Some(valid) = input
                    .rules
                    .iter()
                    .find_map(|r| (r.before == prev).then_some(r.after))
                else {
                    continue 'names;
                };
                if valid & (1 << (ch - b'A')) != 0 {
                    prev = ch;
                } else {
                    continue 'names;
                }
            }
            sum += index;
        }
        sum
    }

    fn part_3(input: &Self::Input) -> usize {
        fn inner(prev: u8, min_len: usize, max_len: usize, input: &Input) -> usize {
            let mut sum = 0;
            if min_len == 0 {
                sum += 1;
            }
            if max_len == 0 {
                return sum;
            }
            let Some(mut valid) = input
                .rules
                .iter()
                .find_map(|r| (r.before == prev).then_some(r.after))
            else {
                return sum;
            };
            while valid != 0 {
                let index = u8::try_from(valid.trailing_zeros()).unwrap();
                valid &= !(1 << index);
                let ch = b'A' + index;
                sum += inner(ch, min_len.saturating_sub(1), max_len - 1, input);
            }
            sum
        }
        let mut count = 0;
        'names: for (name, index) in input.names.iter().zip(0..) {
            if name.len() > 11
                || input
                    .names
                    .iter()
                    .zip(0..)
                    .any(|(nm2, ix2)| ix2 != index && name.starts_with(nm2))
            {
                continue;
            }
            let mut prev = name.as_bytes()[0];
            for ch in name.bytes().skip(1) {
                let Some(valid) = input
                    .rules
                    .iter()
                    .find_map(|r| (r.before == prev).then_some(r.after))
                else {
                    continue 'names;
                };
                if valid & (1 << (ch - b'A')) != 0 {
                    prev = ch;
                } else {
                    continue 'names;
                }
            }
            count += inner(
                prev,
                7_usize.saturating_sub(name.len()),
                11 - name.len(),
                input,
            );
        }
        count
    }
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
