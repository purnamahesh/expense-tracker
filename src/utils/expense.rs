use std::{process::exit, vec};

#[derive(Debug)]
pub struct Expense<'a> {
    amount: f64,
    description: Option<&'a str>,
    category: Option<&'a str>,
    tags: Vec<&'a str>
    // ,datetime TODO
}

impl<'a> Expense<'a> {
    pub fn new(args: &'a Vec<String>) -> Expense<'a> {
        let (amount, description, category, tags) = Self::parse_args(args);
        Expense { amount: amount, description: description, category: category, tags: tags }
    }

    fn parse_args(args: &'a Vec<String>) -> (f64, Option<&str>, Option<&str>, Vec<&str>){
        let mut description: Option<&str> = None;
        let mut amount: f64 = 0.0;
        let mut category: Option<&str> = None;
        let mut tags: Vec<&'a str> = vec![];

        for i in (2..args.len()) {
            if i % 2 == 0 {
                match args[i].as_str() {
                    "--description" => {
                        description = Some(&args[i+1]);
                    },
                    "--amount" => {
                        amount = args[i+1].trim().parse().expect("Error whiel parsing");
                    },
                    "--category" => {
                        category = Some(&args[i+1]);
                    },
                    "--tags" => {
                        tags = args[i+1].split(',').collect();
                    },
                    _ => {
                        println!("Invalid argument {}", args[i]);
                        exit(1)
                    }
                }
            }
        }

        Self::validate_expense(amount, category);

        (amount, description, category, tags)
    }

    fn validate_expense(amount: f64, category: Option<&'a str>) -> bool {
        if amount == 0.0 {
            println!("Amount is required!") ;
            return false;
        }
        else if category.is_none() || category == Some(String::from("").as_str()) {
            println!("category is required!");
            return false;
        }
        return true;
    }
}