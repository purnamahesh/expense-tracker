use std::collections::HashSet;


#[derive(Eq, Hash, PartialEq)]
pub struct Category {
    pub name: String
}


pub struct CategoryList ( pub HashSet<Category> );

impl CategoryList {
    pub fn create_category(&mut self, name: String) -> () {
        let new_cat = Category { name: name };
        self.0.insert(new_cat);
    }

    pub fn print_categories(&self) {
        for cat in &self.0 {
            println!("{}", cat.name);
        }
    }
}