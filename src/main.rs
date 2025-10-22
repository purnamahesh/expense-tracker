use std::{env::args, process::exit};

pub mod utils;
pub mod config;

use crate::utils::{expense::Expense, file_parser::{self, append_expense, read_file}};

fn main() {

    let args:Vec<String> = args().collect();

    // for arg in &args {
    //     println!("{}", arg)
    // }

    if args.len() == 1 {
        println!("HELP")
    } else if args.len() == 2 {
        if args[1] == "help" {
            println!("HELP")
        }
        else if args[1] == "list" {
            read_file(None);
        }
        else if args[1] == "total" {
            
        }
    } else {
        if args.len() % 2 != 0 {
            println!("HELP");
            exit(1)
        }
        else if args[1] == "add" {
            let new_expense = Expense::new(&args);
            append_expense(new_expense.to_csv_record().as_bytes(), None);
        }
        else if args[1] == "filter" {
            
        } else {
            println!("HELP")
        }
    }

    // get_and_prcess_input();
    
}
