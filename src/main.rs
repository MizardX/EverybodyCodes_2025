#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(refining_impl_trait)]

use std::error::Error;
use std::fmt::Display;

use clap::Parser;

mod runner;
use crate::runner::{Cli, Command, Runner};

#[allow(unused)]
trait Day {
    type Input;
    type ParseError: Error;
    fn parse(input: &str) -> Result<Self::Input, Self::ParseError>;

    fn part_1(input: &Self::Input) -> impl Display {
        todo!()
    }

    fn part_2(input: &Self::Input) -> impl Display {
        todo!()
    }

    fn part_3(input: &Self::Input) -> impl Display {
        todo!()
    }
}

// For each day:
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

fn main() {
    let mut runner = Runner::default();
    let cli = Cli::parse();
    if let Some(cmd) = cli.command {
        match cmd {
            Command::Cookie { cookie } => {
                runner.save_cookie(&cookie);
            }
            Command::Download { day } => runner.download(day),
        }
    } else {
        // For each day:

        if cli.day.is_none_or(|d| d == 1) {
            runner.run::<day_01::Day01>(1, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 2) {
            runner.run::<day_02::Day02>(2, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 3) {
            runner.run::<day_03::Day03>(3, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 4) {
            runner.run::<day_04::Day04>(4, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 5) {
            runner.run::<day_05::Day05>(5, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 6) {
            runner.run::<day_06::Day06>(6, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 7) {
            runner.run::<day_07::Day07>(7, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 8) {
            runner.run::<day_08::Day08>(8, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 9) {
            runner.run::<day_09::Day09>(9, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 10) {
            runner.run::<day_10::Day10>(10, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 11) {
            runner.run::<day_11::Day11>(11, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 12) {
            runner.run::<day_12::Day12>(12, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 13) {
            runner.run::<day_13::Day13>(13, cli.part, cli.repeat);
        }

        if cli.day.is_none_or(|d| d == 14) {
            runner.run::<day_14::Day14>(14, cli.part, cli.repeat);
        }

        println!();
    }
}
