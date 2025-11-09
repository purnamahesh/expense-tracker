use clap::Parser;
use std::error::Error;

use expense_tracker::cli;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::ExpenseTrackerArgs::parse();

    cli::parse_sub_commands(args)?;

    Ok(())
}
