use std::io::Write;
use std::fs::OpenOptions;

use crate::config;


pub fn append_expense(content: &[u8], file_path: Option<&str>) {
    let path = file_path.unwrap_or(config::DEFAULT_PATH);
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(path)
    .unwrap_or_else(|err| panic!("Error: {}", err));

    file.write_all(content);
}