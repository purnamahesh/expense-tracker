use std::env;

use assert_cmd::Command;
use rstest::{fixture, rstest};

#[fixture]
fn mock_expenses_file_path() -> String {
    match env::var("GITHUB_WORKSPACE") {
        Ok(path) => path + "/" ,
        Err(_) => String::from("./")
    }
}

#[rstest]
#[should_panic]
fn test_invalid_records_path_file() {
    Command::new("expense-tracker")
        .arg("-p=mock_expenses.psv")
        .arg("list")
        .unwrap();
}

#[rstest]
#[should_panic]
fn test_invalid_records_path_dir() {
    Command::new("expense-tracker")
        .arg("-p=tests/")
        .arg("list")
        .unwrap();
}

#[rstest]
fn test_list(mock_expenses_file_path: String) {
    Command::new("expense-tracker")
        .arg(format!("-p={}tests/resources/mock_expenses.psv", mock_expenses_file_path))
        .arg("list")
        .unwrap();
}

#[rstest]
fn test_total(mock_expenses_file_path: String) {
    Command::new("expense-tracker")
        .arg(format!("-p={}tests/resources/mock_expenses.psv", mock_expenses_file_path))
        .arg("total")
        .unwrap();
}
