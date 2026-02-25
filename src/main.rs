use clap::Parser;
use std::error::Error;

use expense_tracker::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let mut args = cli::ExpenseTrackerArgs::parse();

    args.initialize_db().await?;

    cli::parse_sub_commands(args)?;

    Ok(())
}
