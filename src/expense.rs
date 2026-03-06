use std::error::Error;
// use std::vec;

use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

use crate::cli::FilterArgs;
use crate::constants::TIME_FORMAT;
use crate::models::{ExpenseRecord, ExpenseTotal};

impl ExpenseRecord {
    pub fn new(
        amount: f64,
        description: Option<String>,
        category: String,
        tags: Option<String>,
    ) -> ExpenseRecord {
        ExpenseRecord {
            amount,
            description,
            category,
            tags,
            datetime: Utc::now().timestamp(),
        }
    }

    pub async fn insert_expense_record(&self, conn: Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let _res= sqlx::query("INSERT INTO expense (id, category, amount, tags, datetime, description) values ($1, $2, $3, $4, $5, $6)")
            .bind(Uuid::new_v4().to_string())
            .bind(&self.category)
            .bind(&self.amount)
            .bind(&self.tags)
            .bind(Utc::now().timestamp())
            .bind(&self.description)
            .execute(&conn)
            .await?
            .rows_affected();

        Ok(())
    }

    pub fn display_expenses(expense_list: &Vec<ExpenseRecord>) {
        println!(
            "{:<30}{:<20}{:<10}{:<50}{:<10}",
            "Time", "Category", "Amount", "Description", "Tags"
        );

        for expense in expense_list.iter() {
            println!(
                "{:<30}{:<20}{:<10}{:<50}{:?}",
                DateTime::from_timestamp_secs(expense.datetime)
                    .unwrap()
                    .format(TIME_FORMAT)
                    .to_string(),
                // expense.datetime.format(TIME_FORMAT).to_string(),
                &expense.category,
                &expense.amount,
                &expense
                    .description
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .as_str(),
                &expense.tags
            );
        }
    }

    pub async fn list_expenses(conn: Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
        let expense_list = sqlx::query_as::<_, ExpenseRecord>(
            "SELECT amount, category, datetime, tags, description FROM expense",
        )
        .fetch_all(&conn)
        .await?;

        Self::display_expenses(&expense_list);
        Ok(())
    }

    pub async fn expense_total(conn: Pool<Sqlite>) -> Result<f64, Box<dyn Error>> {
        let expense_total =
            sqlx::query_as::<_, ExpenseTotal>("SELECT sum(amount) total FROM expense")
                .fetch_one(&conn)
                .await?;

        Ok(expense_total.total)
    }

    pub async fn filter_expenses(
        filter_args: FilterArgs,
        conn: Pool<Sqlite>,
    ) -> Result<(), Box<dyn Error>> {
        let query = String::from(
            r#"
                SELECT amount, category, datetime, tags, description 
                FROM expense 
                WHERE ($1 is NULL OR amount = $1)
                AND ($2 is NULL OR category = $2)
                AND ($3 is NULL OR tags LIKE '%$3%')
                AND ($4 is NULL OR amount >= $4)
                AND ($5 is NULL OR amount <= $5)
                LIMIT $6
            "#,
        );

        let expense_list = sqlx::query_as::<_, ExpenseRecord>(&query)
            .bind(&filter_args.amount)
            .bind(&filter_args.category)
            .bind(&filter_args.tag)
            .bind(&filter_args.ge)
            .bind(&filter_args.le)
            .bind(&filter_args.limit.unwrap_or(-1))
            .fetch_all(&conn)
            .await?;

        Self::display_expenses(&expense_list);

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {

//     use directories::ProjectDirs;
//     use rstest::{fixture, rstest};

//     use super::*;

//     const MOCK_EXPENSE_PATH: &str = "tests/resources/mock_expenses.psv";

//     #[fixture]
//     fn mock_expenses_path() -> PathBuf {
//         PathBuf::from(MOCK_EXPENSE_PATH)
//     }

//     #[fixture]
//     fn project_dir() -> PathBuf {
//         let p_dir = ProjectDirs::from("", "", "expense-tracker");
//         p_dir.unwrap().data_dir().to_path_buf()
//     }

//     #[fixture]
//     fn mock_expenses_list(mock_expenses_path: PathBuf, project_dir: PathBuf) -> ExpenseList {
//         let mut mock_expenses_list = ExpenseList::new();
//         mock_expenses_list
//             .load_expenses_from_psv(Some(mock_expenses_path), &Some(project_dir))
//             .unwrap_or_else(|err| {
//                 eprintln!("Failed to parse amount : {}", err);
//             });

//         mock_expenses_list
//     }

//     // #[rstest]
//     // fn test_filter_expense(mock_expenses_list: ExpenseList) {}

//     #[rstest]
//     fn test_expense_total(mock_expenses_path: PathBuf) -> Result<(), Box<dyn Error>> {
//         let total = ExpenseRecord::expense_total(Some(mock_expenses_path))?;
//         assert_eq!(total, 1044.0);
//         Ok(())
//     }
// }
