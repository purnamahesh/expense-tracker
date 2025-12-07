use clap::Parser;
use std::error::Error;

use expense_tracker::cli;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut args = cli::ExpenseTrackerArgs::parse();

    args.set_dirs()?;

    cli::parse_sub_commands(args)?;

    Ok(())
}
