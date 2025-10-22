use std::io::{ErrorKind, Read, Write};
use std::fs::OpenOptions;
use std::process::exit;

use crate::config::{self, HEADER};


pub fn append_expense(content: &[u8], file_path: Option<&str>) {
    let path = file_path.unwrap_or(config::DEFAULT_PATH);
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(path)
    .unwrap_or_else(|err| panic!("Error: {}", err));

    file.write_all(content);
}

pub fn read_file(file_path: Option<&str>) {
    let path = file_path.unwrap_or(config::DEFAULT_PATH);
    let mut file = OpenOptions::new()
    .read(true)
    .open(path)
    .unwrap_or_else(|err| 
        if err.kind() == ErrorKind::NotFound {
            println!("No records yet at {}", path);
            exit(0);
        } else {
            panic!("Error: {}", err)
        }
    );

    let mut content = String::from("");

    file.read_to_string(&mut content);

    println!("{}", HEADER);
    for line in content.trim().split('\n') {
        let x = line.split('|').collect::<Vec<&str>>(); 
        println!(
            "{}, {}, {}, {}",
            x[0], x[1], x[2], x[3]
        );
    }
}