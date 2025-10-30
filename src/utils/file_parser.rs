use std::io::{ErrorKind, Read};
use std::fs::OpenOptions;
use std::process::exit;

use crate::config::{self, HEADER};

pub fn read_file_content(file_path: Option<&str>) -> String {
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

    content
}