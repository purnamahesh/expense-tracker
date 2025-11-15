use std::error::Error;
use std::path::PathBuf;

use clap::{self, Args, Parser, Subcommand};
use directories::ProjectDirs;

use crate::expense::Expense;
use crate::path::{construct_file_path, validate_file_path};

#[derive(Parser, Debug)]
// #[clap(author, version, about)]
// #[command(about = "Does awesome things", long_about = None)]
pub struct ExpenseTrackerArgs {
    #[clap(subcommand)]
    pub command: Operation,
    /// Custom path to psv file where records should be saved.
    #[arg(short='p', long, value_parser = construct_file_path)]
    pub records_path: Option<PathBuf>,
    /// User profile
    #[arg(short, long)]
    pub user: Option<String>,
    data_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
}

impl ExpenseTrackerArgs {
    pub fn set_dirs(&mut self) -> Result<(), &'static str> {
        if let Some(p) = ProjectDirs::from("", "", "expense-tracker") {
            self.config_dir = Some(p.config_dir().to_path_buf());
            self.data_dir = Some(p.data_dir().to_path_buf());
        } else {
            return Err("No valid home directory path detected!");
        };

        Ok(())
    }
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
    /// tags
    #[arg(short, long)]
    pub tag: Option<Vec<String>>,
}

#[derive(Args, Debug)]
#[group(required = true)]
pub struct FilterArgs {
    /// expense amount
    #[arg(short, long)]
    pub amount: Option<f64>,
    /// expense category
    #[arg(short, long)]
    pub category: Option<String>,
    /// tags
    #[arg(short, long)]
    pub tag: Option<Vec<String>>,
}

pub fn parse_sub_commands(args: ExpenseTrackerArgs) -> Result<(), Box<dyn Error>> {
    match args.command {
        Operation::Add(add_args) => {
            let new_expense = Expense::new(
                add_args.amount,
                add_args.description,
                add_args.category,
                add_args.tag,
            );
            new_expense.write_expense_to_psv(args.records_path, &args.data_dir)?;
        }
        Operation::Filter(filter_args) => {
            validate_file_path(&args.records_path)?;
            Expense::filter_expenses(
                filter_args.category,
                filter_args.tag,
                filter_args.amount,
                args.records_path,
                &args.data_dir,
            )?;
        }
        Operation::List => {
            validate_file_path(&args.records_path)?;
            Expense::list_expenses(args.records_path, &args.data_dir)?;
        }
        Operation::Total => {
            validate_file_path(&args.records_path)?;
            let total = Expense::expense_total(args.records_path, &args.data_dir)?;
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
