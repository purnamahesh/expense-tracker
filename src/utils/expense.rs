use std::collections::HashMap;
use std::num::ParseFloatError;
use std::{process::exit, vec, io::Write};
use std::fs::OpenOptions;

use crate::utils::file_parser::read_file_content;
use crate::config::DEFAULT_PATH;


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
        let content: String = read_file_content(file_path);
        for line in content.trim().split('\n') {
            let fields:Vec<&str> = line.trim().split('|').collect() ;
            self.expense_list.push(
                Expense {
                    amount: fields[1].trim().parse::<f64>()?,
                    description: Some(fields[2][1..fields[2].len()-1].to_owned()),
                    category: fields[0].to_owned(), 
                    tags: fields[3][1..fields[3].len()-1]
                    .trim().split(',').map(|s| s.to_string()).collect()
                }
            );
        }

        Ok(())
    }

}


impl Expense {

    pub fn new(amount:f64, description:Option<String>, category:String, tags:Vec<String>) -> Expense {
        if ! Self::validate_expense(amount, &category) {
            exit(1)
        }
        Expense { amount: amount, description: description, category: category, tags: tags }
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

    pub fn write_expense_to_psv(&self, file_path: Option<&str>) {

        let record = self.to_psv_record();
        let content = record.as_bytes();
    
        let path = file_path.unwrap_or(DEFAULT_PATH);
        
        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|err| panic!("Error: {}", err));

        file.write_all(content).unwrap_or_else(|err| {
            eprintln!("Error write to file: {}", err);
            exit(1);
        });
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

    pub fn display_expenses(expense_list: Vec<&Expense>
    ) {
        println!("{:<20}{:<10}{:<50}{:<10}", "Category", "Amount", "Description", "Tags");

        for expense in expense_list.iter() {
            println!(
                "{:<20}{:<10}{:<50}{:?}",
                expense.category,
                expense.amount, 
                expense.description.as_ref().unwrap_or(&"".to_string()).as_str(), 
                expense.tags
            );
        }
    }

    pub fn list_expenses() {

        let expense_list = Self::get_expense_list_from_psv(None);
        let x = Vec::from_iter(expense_list.expense_list.iter().map(|x| x)) ;

        Self::display_expenses(
            x
        );
    }

    pub fn expense_total() -> Result<(), Box<dyn std::error::Error>> {

        let expense_list = Self::get_expense_list_from_psv(None);

        let mut total:f64 = 0.0;

        for expense in expense_list.expense_list {
            total += expense.amount;
        }

        println!("Total: {}", total);

        Ok(())
    }

    fn get_expense_list_from_psv(file_path: Option<&str>) -> ExpenseList {
        let mut expense_list = ExpenseList::new();

        expense_list.load_expenses_from_psv(file_path).unwrap_or_else(|err| {
            eprintln!("Error : {}", err);
            exit(1);
        });

        expense_list
    }

    pub fn filter_expenses(filters: HashMap<String, String>) {

        let expense_list = Self::get_expense_list_from_psv(None);

        let filtered_list = expense_list.expense_list.iter().filter(|exp| {
            let amount_flag = if filters.get("amount").is_some(){ filters.get("amount").unwrap().trim() == exp.amount.to_string() } else { true };
            let category_flag = if filters.get("category").is_some(){ filters.get("category").unwrap().trim() == exp.category } else { true };
            amount_flag && category_flag
        });
        let x = filtered_list.collect::<Vec<&Expense>>();

        Self::display_expenses(x);
    }
    
}