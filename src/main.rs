use std::fmt::Display;

use clap::Parser;

use crate::day_01::Day01;

trait Day {
    type Output1: Display;
    type Output2: Display;
    type Output3: Display;

    fn part_1(input: &str) -> Self::Output1;
    fn part_2(input: &str) -> Self::Output2 { todo!() }
    fn part_3(input: &str) -> Self::Output3 { todo!() }
}

mod day_01;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    day: Option<usize>,
    #[arg(short, long)]
    part: Option<usize>,
}

fn run<D: Day>(num: usize, part: Option<usize>) {
    println!();
    println!("Day {num}");
    if part.is_none_or(|p| p == 1) {
        let input = std::fs::read_to_string(format!("./input/day_{num:02}_part_1.txt")).unwrap();
        println!("  Part 1: {}", D::part_1(&input));
    }
    if part.is_none_or(|p| p == 2) {
        let input = std::fs::read_to_string(format!("./input/day_{num:02}_part_2.txt")).unwrap();
        println!("  Part 2: {}", D::part_2(&input));
    }
    if part.is_none_or(|p| p == 3) {
        let input = std::fs::read_to_string(format!("./input/day_{num:02}_part_3.txt")).unwrap();
        println!("  Part 3: {}", D::part_3(&input));
    }
}

const DAYS: &[fn(Option<usize>)] = &[|part| run::<Day01>(1, part)];

fn main() {
    let cli = Cli::parse();

    for (num, day) in (1..).zip(DAYS) {
        if cli.day.is_some_and(|day| day != num) {
            continue;
        }
        day(cli.part);
    }
    println!();
}
