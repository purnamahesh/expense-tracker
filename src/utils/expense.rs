use std::num::ParseFloatError;
use std::{process::exit, vec};

use crate::utils::file_parser::read_file_content;
use crate::config::{HEADER, DEFAULT_PATH};


#[derive(Debug)]
pub struct Expense {
    amount: f64,
    description: Option<String>,
    category: String,
    tags: Vec<String>
    // ,datetime TODO
}

pub struct ExpenseList {
    expense_list: Vec<Expense>
}

impl ExpenseList {

    pub fn new() -> Self {
        ExpenseList { expense_list: vec![] }
    }

    pub fn load_expenses_from_psv(&mut self, file_path: Option<&str>) -> Result<(), ParseFloatError>{
        let content: String = read_file_content(None);
        for line in content.trim().split('\n') {
            let fields:Vec<&str> = line.trim().split('|').collect() ;
            self.expense_list.push(
                Expense { 
                    amount: fields[1].trim().parse::<f64>()?, 
                    description: Some(fields[2].to_owned()),
                    category: fields[0].to_owned(), 
                    tags: fields[3].trim().split(',')
                    .map(|s| s.to_string()).collect()
                }
            );
        }
        // self.content = content;
        Ok(())
    }

}


impl Expense {
    pub fn new(args: Vec<String>) -> Expense {
        let (amount, description, category, tags) = Self::add_parse_args(args);
        if ! Self::validate_expense(amount, &category) {
            exit(1)
        }
        Expense { amount: amount, description: description, category: category, tags: tags }
    }

    fn add_parse_args(args: Vec<String>) -> (f64, Option<String>, String, Vec<String>){
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
                        tags = args[i+1].split(',').map(|s| s.to_string()).collect();
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

    fn validate_expense(amount: f64, category: &str) -> bool {
        if amount == 0.0 {
            eprintln!("Amount is required!") ;
            return false;
        }
        else if category == "" {
            eprintln!("category is required!");
            return false;
        }
        return true;
    }

    pub fn to_psv_record(&self) -> String {
        format!(
            "{}|{}|\"{}\"|\"{}\"\n", 
            self.category.as_str(),
            self.amount, 
            self.description.as_ref().unwrap_or(&"".to_string()), 
            self.tags.join(",")
        )
    }

    pub fn list_expenses() {

        let content: String = read_file_content(None);

        println!("{}", HEADER);
        for line in content.trim().split('\n') {
            let x = line.split('|').collect::<Vec<&str>>(); 
            println!(
                "{}, {}, {}, {}",
                x[0], x[1], x[2], x[3]
            );
        }
    }

    pub fn expense_total() -> Result<(), Box<dyn std::error::Error>> {

        let content = read_file_content(None);

        let mut total:f64 = 0.0;

        for line in content.trim().split('\n') {
            let x = line.split('|').collect::<Vec<&str>>(); 
            total += x[1].trim().parse::<f64>()?;
        }

        println!("Total: {}", total);

        Ok(())
    }
 
    
}