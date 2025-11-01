mod utils;
mod config;
mod cli;

use clap::{Parser};

use crate::cli::parse_sub_commands;

fn main() {
    let args = cli::ExpenseTrackerArgs::parse();

    parse_sub_commands(args);
}