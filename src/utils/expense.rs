use std::fs::OpenOptions;
use std::num::ParseFloatError;
use std::path::{Path, PathBuf};
use std::{io::Write, process::exit, vec};

use chrono::{DateTime, Utc};

use crate::config::TIME_FORMAT;
use crate::utils::file_parser::read_file_content;

pub struct Expense {
    amount: f64,
    category: String,
    tags: Vec<String>,
    datetime: DateTime<Utc>,
    description: Option<String>,
}

pub struct ExpenseList {
    expense_list: Vec<Expense>,
}

impl ExpenseList {
    pub fn new() -> Self {
        ExpenseList {
            expense_list: vec![],
        }
    }

    pub fn load_expenses_from_psv(
        &mut self,
        file_path: Option<PathBuf>,
    ) -> Result<(), ParseFloatError> {
        let content: String = read_file_content(file_path);
        for line in content.trim().split('\n') {
            let fields: Vec<&str> = line.trim().split('|').collect();
            self.expense_list.push(Expense {
                amount: fields[2].trim().parse::<f64>()?,
                description: Some(fields[3][1..fields[3].len() - 1].to_owned()),
                category: fields[1].to_owned(),
                tags: fields[4][1..fields[4].len() - 1]
                    .trim()
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                datetime: DateTime::parse_from_str(fields[0], TIME_FORMAT)
                    .unwrap_or_else(|err| {
                        eprintln!("Error parsing date: {}", err);
                        exit(1)
                    })
                    .to_utc(),
            });
        }

        Ok(())
    }
}

impl Expense {
    pub fn new(
        amount: f64,
        description: Option<String>,
        category: String,
        tags: Option<Vec<String>>,
    ) -> Expense {
        Expense {
            amount,
            description,
            category,
            tags: tags.unwrap_or_default(),
            datetime: Utc::now(),
        }
    }

    pub fn write_expense_to_psv(&self, file_path: Option<PathBuf>) {
        let record = self.to_psv_record();
        let content = record.as_bytes();

        let path = file_path.unwrap_or(Path::new("expense_db.psv").to_path_buf());

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
            "{}|{}|{}|\"{}\"|\"{}\"\n",
            self.datetime.format(TIME_FORMAT),
            self.category.as_str().to_lowercase(),
            self.amount,
            self.description.as_ref().unwrap_or(&"".to_string()),
            self.tags.join(",").to_lowercase()
        )
    }

    pub fn display_expenses(expense_list: Vec<&Expense>) {
        println!(
            "{:<30}{:<20}{:<10}{:<50}{:<10}",
            "Time", "Category", "Amount", "Description", "Tags"
        );

        for expense in expense_list.iter() {
            println!(
                "{:<30}{:<20}{:<10}{:<50}{:?}",
                expense.datetime.format(TIME_FORMAT).to_string(),
                expense.category,
                expense.amount,
                expense
                    .description
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .as_str(),
                expense.tags
            );
        }
    }

    pub fn list_expenses(file_path: Option<PathBuf>) {
        let expense_list = Self::get_expense_list_from_psv(file_path);
        let x = Vec::from_iter(expense_list.expense_list.iter());

        Self::display_expenses(x);
    }

    pub fn expense_total(file_path: Option<PathBuf>) -> Result<f64, Box<dyn std::error::Error>> {
        let expense_list = Self::get_expense_list_from_psv(file_path);

        let mut total: f64 = 0.0;

        for expense in expense_list.expense_list {
            total += expense.amount;
        }

        Ok(total)
    }

    fn get_expense_list_from_psv(file_path: Option<PathBuf>) -> ExpenseList {
        let mut expense_list = ExpenseList::new();

        expense_list
            .load_expenses_from_psv(file_path)
            .unwrap_or_else(|err| {
                eprintln!("Error : {}", err);
                exit(1);
            });

        expense_list
    }

    pub fn filter_expenses(
        category: Option<String>,
        tags: Option<Vec<String>>,
        amount: Option<f64>,
        file_path: Option<PathBuf>,
    ) {
        let expense_list = Self::get_expense_list_from_psv(file_path);

        let filtered_list = expense_list.expense_list.iter().filter(|exp: &&Expense| {
            let amount_flag = if let Some(amount) = amount {
                amount == exp.amount
            } else {
                true
            };

            let category_flag = if let Some(category) = &category {
                category.as_str() == exp.category.as_str()
            } else {
                true
            };

            let tags_flag = if let Some(tags) = &tags {
                let mut flag = false;
                if exp.tags.is_empty() {
                    flag = false
                } else {
                    for tag in tags {
                        flag = exp.tags.contains(tag);
                    }
                }
                flag
            } else {
                true
            };

            amount_flag && category_flag && tags_flag
        });
        let x = filtered_list.collect::<Vec<_>>();

        Self::display_expenses(x);
    }
}

#[cfg(test)]
mod tests {

    use rstest::fixture;
    use rstest::rstest;

    use super::*;

    const MOCK_EXPENSE_PATH: &'static str = "tests/resources/mock_expenses.psv";

    #[fixture]
    fn mock_expenses_list() -> ExpenseList {
        let mut mock_expenses_list = ExpenseList::new();
        mock_expenses_list.load_expenses_from_psv(Some(MOCK_EXPENSE_PATH));

        mock_expenses_list
    }

    #[rstest]
    fn test_filter_expense(mock_expenses_list: ExpenseList) {
        Expense::display_expenses(
            mock_expenses_list
                .expense_list
                .iter()
                .collect::<Vec<&Expense>>(),
        );
    }

    #[rstest]
    fn test_expense_total(mock_expenses_list: ExpenseList) {
        let total = Expense::expense_total(Some(MOCK_EXPENSE_PATH));
        match total {
            Ok(total) => {
                println!("Total: {}", total)
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                exit(1);
            }
        }
        // display_expenses(mock_expenses_list.expense_list.iter().collect::<Vec<&Expense>>());
    }
}
