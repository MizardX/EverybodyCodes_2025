use std::error::Error;
use std::fmt::Display;

use clap::Parser;

mod runner;
use crate::runner::{Cli, Command, Runner};

#[allow(unused)]
trait Day {
    type Input;
    type ParseError: Error;
    type Output1: Display;
    type Output2: Display;
    type Output3: Display;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError>;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        todo!()
    }
    fn part_2(input: &Self::Input) -> Self::Output2 {
        todo!()
    }
    fn part_3(input: &Self::Input) -> Self::Output3 {
        todo!()
    }
}

// For each day:
mod day_01;
mod day_02;

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

        println!();
    }
}