#[cfg(all(feature = "full", feature = "lite"))]
compile_error!("Features `full` and `lite` are mutually exclusive. Please enable only one.");

use clap::Parser;
use std::error::Error;

use pense::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = cli::ExpenseTrackerArgs::parse();

    cli::parse_sub_commands(args).await?;

    Ok(())
}
