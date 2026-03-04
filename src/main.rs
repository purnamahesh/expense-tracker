use clap::Parser;
use pense::sqlite_conn::initialize_db;
use std::error::Error;

use pense::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let args = cli::ExpenseTrackerArgs::parse();

    let conn = initialize_db().await?;

    cli::parse_sub_commands(args, conn).await?;

    Ok(())
}
