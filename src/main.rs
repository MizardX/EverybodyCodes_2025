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

#[tokio::main]
async fn main() {
    let mut runner = Runner::default();
    let cli = Cli::parse();
    if let Some(cmd) = cli.command {
        match cmd {
            Command::Cookie { cookie } => {
                runner.save_cookie(&cookie);
            }
            Command::Download { day } => runner.download(day).await,
        }
    } else {
        // For each day:

        if cli.day.is_none_or(|d| d == 1) {
            runner.run::<day_01::Day01>(1, cli.part).await;
        }

        if cli.day.is_none_or(|d| d == 2) {
            runner.run::<day_02::Day02>(2, cli.part).await;
        }

        if cli.day.is_none_or(|d| d == 3) {
            runner.run::<day_03::Day03>(3, cli.part).await;
        }

        if cli.day.is_none_or(|d| d == 4) {
            runner.run::<day_04::Day04>(4, cli.part).await;
        }

        if cli.day.is_none_or(|d| d == 5) {
            runner.run::<day_05::Day05>(5, cli.part).await;
        }

        if cli.day.is_none_or(|d| d == 6) {
            runner.run::<day_06::Day06>(6, cli.part).await;
        }

        println!();
    }
}