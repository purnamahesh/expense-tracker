mod cli;
mod config;
mod utils;

use clap::Parser;

use crate::cli::parse_sub_commands;

use std::env::{self, home_dir};
use std::fs::DirBuilder;
use std::path::Path;
use std::process::exit;

fn main() {
    let args = cli::ExpenseTrackerArgs::parse();

    parse_sub_commands(args);

    // let x = Path::

    let config_dir = match env::home_dir() {
        Some(home_dir) => {
            let config_dir: std::path::PathBuf = home_dir.join(Path::new("expense-tracker/"));
            if !config_dir.exists() {
                DirBuilder::new()
                    .recursive(false)
                    .create(config_dir.as_path())
                    .unwrap_or_else(|err| {
                        eprintln!(
                            "Error creating directory at {}\nError: {}",
                            config_dir.display(),
                            err
                        );
                        exit(1)
                    });
            };
            config_dir
        }
        None => {
            eprint!("Could'nt locate home directory");
            exit(1);
        }
    };

    let db_file = config_dir.join(Path::new("expense_db.psv"));

    let y = match db_file.exists() {
        true => {}
        false => {}
    };

    // let y = Path::new(home_dir);

    // println!("{}", x.display());
}
