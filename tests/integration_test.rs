use std::env;

use assert_cmd::Command;
use assert_cmd::cargo;
use rstest::{fixture, rstest};

#[fixture]
fn mock_expenses_file_path() -> String {
    match env::var("GITHUB_WORKSPACE") {
        Ok(path) => path + "/",
        Err(_) => String::from("./"),
    }
}

#[rstest]
fn test_invalid_records_path_file() {
    Command::new(cargo::cargo_bin!("pense"))
        .arg("-p=./tests/pense.txt")
        .arg("list")
        .assert()
        .failure();
}

#[rstest]
fn test_invalid_records_path_dir() {
    Command::new(cargo::cargo_bin!("pense"))
        .arg("-p=tests/")
        .arg("list")
        .assert()
        .failure();
}

#[rstest]
fn test_list(mock_expenses_file_path: String) {
    Command::new(cargo::cargo_bin!("pense"))
        .arg(format!(
            "-p={}tests/resources/pense.db",
            mock_expenses_file_path
        ))
        .arg("list")
        .assert()
        .success();
}

#[rstest]
fn test_total(mock_expenses_file_path: String) {
    Command::new(cargo::cargo_bin!("pense"))
        .arg(format!(
            "-p={}tests/resources/pense.db",
            mock_expenses_file_path
        ))
        .arg("total")
        .assert()
        .stdout("Total: 594\n");
}
