use std::{env::args, process::exit};

pub mod utils;
pub mod config;

use crate::utils::{expense::{Expense, ExpenseList}, file_parser::append_expense};

fn main() {

    let args:Vec<String> = args().collect();

    if args.len() == 1 {
        println!("Usage")
    } else if args.len() == 2 {
        if args[1] == "help" {
            println!("Usage")
        }
        else if args[1] == "list" {
            Expense::list_expenses();
        }
        else if args[1] == "total" {
            Expense::expense_total()
            .unwrap_or_else(|err| panic!("Error: {}", err));
        } else {
            println!("Usage")
        }
    } else {
        if args.len() % 2 != 0 {
            println!("Usage");
            exit(1)
        }
        else if args[1] == "add" {
            let new_expense = Expense::new(&args);
            append_expense(new_expense.to_psv_record().as_bytes(), None);
        }
        else if args[1] == "filter" {
            ExpenseList::new();
        } else {
            println!("Usage")
        }
    }
    
}
