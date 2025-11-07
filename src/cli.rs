use std::{path::PathBuf, process::exit};

use clap::{self, Args, Parser, Subcommand};

use crate::utils::expense::Expense;
use crate::utils::path::construct_abs_path;

#[derive(Parser, Debug)]
// #[clap(author, version, about)]
// #[command(about = "Does awesome things", long_about = None)]
pub struct ExpenseTrackerArgs {
    #[clap(subcommand)]
    pub command: Operation,
    /// Custom path to psv file where records should be saved.
    #[arg(short='p', long, value_parser = validate_file_path)]
    pub records_path: Option<PathBuf>,
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

fn validate_file_path(arg: &str) -> Result<PathBuf, &'static str> {
    let actual_path = construct_abs_path(arg);

    if !actual_path.exists() {
        return Err("File doesn't exist");
    } else if !actual_path.is_file() {
        return Err("Not a file");
    }

    Ok(actual_path)
}

fn amount_validation(arg: &str) -> Result<f64, &'static str> {
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
    #[arg(short, long, value_parser = amount_validation)]
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

pub fn parse_sub_commands(args: ExpenseTrackerArgs) {
    match args.command {
        Operation::Add(add_args) => {
            let new_expense = Expense::new(
                add_args.amount,
                add_args.description,
                add_args.category,
                add_args.tag,
            );
            new_expense.write_expense_to_psv(args.records_path);
        }
        Operation::Filter(filter_args) => {
            Expense::filter_expenses(
                filter_args.category,
                filter_args.tag,
                filter_args.amount,
                args.records_path,
            );
        }
        Operation::List => {
            Expense::list_expenses(args.records_path);
        }
        Operation::Total => {
            let total = Expense::expense_total(args.records_path);
            match total {
                Ok(total) => {
                    println!("Total: {}", total)
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[should_panic]
    fn test_add_command() {
        panic!("error")
    }
}
