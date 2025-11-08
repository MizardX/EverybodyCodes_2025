use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};
use std::str::FromStr;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use thiserror::Error;

use crate::Day;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex {
    x: i64,
    y: i64,
}

impl Complex {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn exceeds(self, limit: u64) -> bool {
        self.x.unsigned_abs() > limit || self.y.unsigned_abs() > limit
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        (self.x, self.y) = (
            self.x * rhs.x - self.y * rhs.y,
            self.x * rhs.y + self.y * rhs.x,
        );
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl DivAssign<i64> for Complex {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Div<i64> for Complex {
    type Output = Self;

    fn div(mut self, rhs: i64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl FromStr for Complex {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('[')
            .ok_or(ParseError::SyntaxError)?
            .strip_suffix(']')
            .ok_or(ParseError::SyntaxError)?
            .split_once(',')
            .ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

pub struct Day02;

impl Day for Day02 {
    type Input = Complex;
    type ParseError = ParseError;
    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input
            .strip_prefix("A=")
            .ok_or(ParseError::SyntaxError)?
            .parse()
    }

    fn part_1(&input: &Self::Input) -> Complex {
        let mut result = Complex::new(0, 0);
        for _ in 0..3 {
            result *= result;
            result /= 10;
            result += input;
        }
        result
    }

    fn part_2(&input: &Self::Input) -> usize {
        let mut count = 0;
        for x in (input.x..=input.x + 1000).step_by(10) {
            'y: for y in (input.y..=input.y + 1000).step_by(10) {
                let z = Complex::new(x, y);
                let mut m = Complex::new(0, 0);
                for _ in 0..100 {
                    m *= m;
                    m /= 100000;
                    m += z;
                    if m.exceeds(1000000) {
                        continue 'y;
                    }
                }
                count += 1;
            }
        }
        count
    }

    fn part_3(&input: &Self::Input) -> usize {
        (0..1001 * 1001)
            .into_par_iter()
            .filter(|&xy| {
                let m = Complex::new(xy % 1001 + input.x, xy / 1001 + input.y);
                let mut z = Complex::new(0, 0);
                for _ in 0..100 {
                    z *= z;
                    z /= 100_000;
                    z += m;
                    if z.exceeds(1_000_000) {
                        return false;
                    }
                }
                true
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgb};

    use super::*;

    const EXAMPLE1: &str = "A=[25,9]";
    const EXAMPLE2: &str = "A=[35300,-64910]";

    #[test]
    fn test_parse() {
        let result = Day02::parse(EXAMPLE1).unwrap();
        assert_eq!(result, Complex::new(25, 9));
    }

    #[test]
    fn test_part_1() {
        let input = Day02::parse(EXAMPLE1).unwrap();
        let result = Day02::part_1(&input);
        assert_eq!(result, Complex::new(357, 862));
    }

    #[test]
    fn test_part_2() {
        let input = Day02::parse(EXAMPLE2).unwrap();
        let result = Day02::part_2(&input);
        assert_eq!(result, 4_076);
    }

    #[test]
    fn test_part_3() {
        let input = Day02::parse(EXAMPLE2).unwrap();
        let result = Day02::part_3(&input);
        assert_eq!(result, 406_954);
    }

    #[test]
    #[ignore = "Generates image"]
    fn test_render() {
        let input = Day02::parse(EXAMPLE2).unwrap();
        let pixels = (0..1001 * 1001)
            .into_par_iter()
            .filter_map(|xy| {
                let m = Complex::new(xy % 1001 + input.x, xy / 1001 + input.y);
                let mut z = Complex::new(0, 0);
                for t in 0_u8..=255 {
                    z *= z;
                    z /= 100_000;
                    z += m;
                    if z.exceeds(1_000_000) {
                        return Some((xy, t, z.x.unsigned_abs().max(z.y.unsigned_abs())));
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        let mut image = ImageBuffer::<Rgb<u8>, _>::new(1001, 1001);
        for &(xy, t, _dist) in &pixels {
            let clr = u8::try_from(unsafe {
                ((f64::from(t) / 255.0).sqrt() * 255.0).to_int_unchecked::<i64>()
            })
            .unwrap_or(255);
            image.put_pixel(
                u32::try_from(xy % 1001).unwrap(),
                u32::try_from(xy / 1001).unwrap(),
                Rgb([clr, clr, clr]),
            );
        }
        let filename = "input/day_02.png";
        image
            .save_with_format(filename, image::ImageFormat::Png)
            .unwrap();
        println!("Saved image to {filename}");
    }
}
