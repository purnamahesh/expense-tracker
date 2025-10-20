use crate::utils::category::Category;
use crate::utils::tags::Tag;
use crate::utils::stdin_utils;

pub struct Expense {
    amount: f64,
    description: String,
    category: Category,
    tags: Vec<Tag>
}

impl Expense {
    fn get_expense(&self) {
        let amount = stdin_utils::read_f64("Enter amount:");
        let desc = stdin_utils::read_sting("Enter Description:");
        // let amount = stdin_utils::get_index("Choose Category:"); //TODO
        // let tags = stdin_utils::add_tags("Enter amount:");
    }
}