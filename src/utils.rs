pub mod expense;
pub mod category;
pub mod tags;
pub mod stdin_utils;

use crate::utils::{category::{CategoryList}};
use std::{collections::HashSet, process::exit};


pub fn get_and_prcess_input() {

    let mut categories: CategoryList =  CategoryList ( HashSet::new() );

    println!("------------------------------------------");
    println!("Personal Expense Tracker");
    println!("------------------------------------------");
    

    while true {

        let option = stdin_utils::read_u8("1. Enter Expense Amount\n2. Create Category\n3. Create Tag\n4. List expenses\n");

        match option {
            1 => {
                // expense::Expense
            },
            2 => {
                // create category
                let cat_name = stdin_utils::read_sting("Enter category name: ");
                categories.create_category(cat_name);

            },
            3 => {
                // create tag
                categories.print_categories();
            },
            4 => {
                // list expenses
            },
            _ => {
                println!("Invalid Option");
                exit(1);
            }
        }
    }
}