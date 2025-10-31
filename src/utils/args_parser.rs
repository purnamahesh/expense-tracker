use std::collections::HashMap;
use std::process::exit;
use std::env::args;
use crate::utils::expense::Expense;

pub fn filter_parse_args() -> HashMap<String, String> {
    let args:Vec<String> = args().collect();

    let mut filters:HashMap<String, String> = HashMap::new();

    for i in 2..args.len() {
        if i % 2 == 0 {
            match args[i].as_str() {
                "--amount" => {
                    filters.insert(
                        "amount".to_string(), 
                        args[i+1].trim().to_string()
                    );
                },
                "--category" => {
                    filters.insert(
                        "category".to_string(), 
                        args[i+1].trim().to_string()
                    );
                },
                "--tags" => {
                    filters.insert(
                        "tags".to_string(), 
                        args[i+1].trim().to_string()
                    );
                },
                "--description" => {
                    eprintln!("Description is not filterable!");
                    exit(1);
                },
                _ => {
                    println!("Invalid argument {}", args[i]);
                    exit(1)
                }
            }
        }
    }

    filters
}


pub fn add_parse_args() -> (f64, Option<String>, String, Vec<String>){

    let args:Vec<String> = args().collect();

    let mut description: Option<String> = None;
    let mut amount: f64 = 0.0;
    let mut category: String = "".to_string();
    let mut tags: Vec<String> = vec![];

    for i in 2..args.len() {
        if i % 2 == 0 {
            match args[i].as_str() {
                "--description" => {
                    description = Some(args[i+1].to_owned());
                },
                "--amount" => {
                    amount = args[i+1].trim().parse().expect("Error whiel parsing");
                },
                "--category" => {
                    category = args[i+1].to_owned();
                },
                "--tags" => {
                    tags = args[i+1].trim().split(',').map(|s| s.to_string()).collect();
                },
                _ => {
                    println!("Invalid argument {}", args[i]);
                    exit(1)
                }
            }
        }
    }

    (amount, description, category, tags)
}


pub fn parse_args() {
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
            let (amount, description, category, tags) = add_parse_args();
            let new_expense = Expense::new(amount, description, category, tags);
            new_expense.write_expense_to_psv(None);
        }
        else if args[1] == "filter" {
            let filters = filter_parse_args();
            Expense::filter_expenses(filters);
        } else {
            println!("Usage")
        }
    }
}