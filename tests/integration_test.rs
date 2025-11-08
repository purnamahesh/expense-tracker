use assert_cmd::Command;
use rstest::rstest;

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
fn test_list() {
    Command::new("expense-tracker")
        .arg("-p=tests/resources/mock_expenses.psv")
        .arg("list")
        .unwrap();
}

#[rstest]
fn test_total() {
    Command::new("expense-tracker")
        .arg("-p=tests/resources/mock_expenses.psv")
        .arg("total")
        .unwrap();
}
