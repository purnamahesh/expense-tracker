use std::fs::OpenOptions;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::process::exit;

use crate::config::FILE_NAME;

pub fn read_file_content(file_path: Option<PathBuf>) -> String {
    let path = file_path.unwrap_or(Path::new(FILE_NAME).to_path_buf());
    let mut file = OpenOptions::new()
        .read(true)
        .open(&path)
        .unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                eprintln!("No records yet at {}", path.display());
                exit(0);
            } else {
                eprintln!("Error : {}", err);
                exit(0);
            }
        });

    let mut content = String::from("");

    file.read_to_string(&mut content).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        exit(1);
    });

    content
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     #[should_panic(expected="No records yet at ")]
//     fn test_load_expenses_from_psv() {
//         let _ = read_file_content(Some("./expense_db.csv"));
//     }
// }
