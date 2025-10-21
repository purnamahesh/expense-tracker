use std::{env::args, fs::File, io::{self, Error, ErrorKind}, process::exit};

pub mod utils;

use crate::utils::{expense::Expense};

fn main() {

    let args:Vec<String> = args().collect();

    if args.len() == 1 {
        println!("HELP")
    } else if args.len() == 2 {
        if args[1] == "help" {
            println!("HELP")
        }
        else if args[1] == "list" {
            
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
            dbg!(new_expense);
        }
        else if args[1] == "filter" {
            
        } else {
            println!("HELP")
        }
    }

    // get_and_prcess_input();
    
}

