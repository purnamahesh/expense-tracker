mod cli;
mod config;
mod utils;

use clap::Parser;

use crate::cli::parse_sub_commands;

fn main() {
    let args = cli::ExpenseTrackerArgs::parse();

    parse_sub_commands(args);
}
