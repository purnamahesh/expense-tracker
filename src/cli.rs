use clap::{self, Args, Parser, Subcommand};

use crate::utils::expense::Expense;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
// #[command(about = "Does awesome things", long_about = None)]
pub struct ExpenseTrackerArgs {
    #[clap(subcommand)]
    pub command: Operation,
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

#[derive(Args, Debug)]
pub struct AddArgs {
    /// expense amount
    #[arg(short, long)]
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
            new_expense.write_expense_to_psv(None);
        }
        Operation::Filter(filter_args) => {
            Expense::filter_expenses(filter_args.category, filter_args.tag, filter_args.amount);
        }
        Operation::List => {
            Expense::list_expenses();
        }
        Operation::Total => {
            Expense::expense_total().unwrap_or_else(|err| panic!("Error: {}", err));
        }
    }
}
