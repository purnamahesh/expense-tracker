use clap::Parser;

use expense_tracker::cli;

fn main() {
    let args = cli::ExpenseTrackerArgs::parse();

    cli::parse_sub_commands(args);
}
