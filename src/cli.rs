use std::error::Error;
use std::path::PathBuf;

use crate::sqlite_conn::initialize_db;
use clap::{self, Args, Parser, Subcommand};

use crate::{models::ExpenseRecord, path::construct_file_path};

#[derive(Parser, Debug)]
// #[clap(author, version, about)]
// #[command(about = "Does awesome things", long_about = None)]
pub struct ExpenseTrackerArgs {
    #[clap(subcommand)]
    pub command: Operation,
    /// Custom path to sqlitedb file where records should be saved.
    #[arg(short='p', long, value_parser = construct_file_path)]
    pub records_path: Option<PathBuf>,
    /// User profile
    #[arg(short, long)]
    pub user: Option<String>,
    // data_dir: Option<PathBuf>,
    // config_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// add expense record
    Add(AddArgs),
    /// filter expense records
    Filter(FilterArgs),
    /// list expense records
    List,
    /// Expense total
    Total,
}

fn validate_amount(arg: &str) -> Result<f64, &'static str> {
    let amount = arg.parse::<f64>();
    match amount {
        Ok(amount) => match amount > 0.0 {
            true => Ok(amount),
            false => Err("amount should be greater than 0.0"),
        },
        Err(_err) => Err("Invalid value for amount!"),
    }
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// expense amount
    #[arg(short, long, value_parser = validate_amount)]
    pub amount: f64,
    /// expense category
    #[arg(short, long)]
    pub category: String,
    /// Description
    #[arg(short, long)]
    pub description: Option<String>,
    /// comma seperated tags eg: tag1,tag2
    #[arg(short, long)]
    pub tag: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = true)]
pub struct FilterArgs {
    /// amount equals
    #[arg(short, long)]
    pub amount: Option<f64>,
    /// expense category
    #[arg(short, long)]
    pub category: Option<String>,
    /// tags
    #[arg(short, long)]
    pub tag: Option<String>,
    /// amount greater than and equal to
    #[arg(long)]
    pub ge: Option<f64>,
    /// amount less than and equal to
    #[arg(long)]
    pub le: Option<f64>,
    /// limit fetched records
    #[arg(short, long)]
    pub limit: Option<i64>,
}

pub async fn parse_sub_commands(args: ExpenseTrackerArgs) -> Result<(), Box<dyn Error>> {
    let conn = initialize_db(
        args.records_path,
        matches!(&args.command, Operation::Add(_))
        // match &args.command {
        //     Operation::Add(_) => true,
        //     _ => false,
        // },
    )
    .await?;
    match args.command {
        Operation::Add(add_args) => {
            let new_expense = ExpenseRecord::new(
                add_args.amount,
                add_args.description,
                add_args.category,
                add_args.tag,
            );
            new_expense.insert_expense_record(conn).await?;
        }
        Operation::Filter(filter_args) => {
            ExpenseRecord::filter_expenses(filter_args, conn).await?;
        }
        Operation::List => {
            ExpenseRecord::list_expenses(conn).await?;
        }
        Operation::Total => {
            let total = ExpenseRecord::expense_total(conn).await?;
            println!("Total: {}", total);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::rstest;

    // Testing validate_amount
    #[rstest]
    #[case("0.186", 0.186)]
    #[case("10", 10.0)]
    #[case("12.5", 12.5)]
    fn test_validate_amount_valid(#[case] amount_inp: &str, #[case] amount_out: f64) {
        assert_eq!(validate_amount(amount_inp), Ok(amount_out))
    }

    #[rstest]
    #[case("0", "amount should be greater than 0.0")]
    #[case("10doller", "Invalid value for amount!")]
    fn test_validate_amount_invalid(#[case] amount_inp: &str, #[case] err_str: &str) {
        assert_eq!(validate_amount(amount_inp), Err(err_str))
    }
}
