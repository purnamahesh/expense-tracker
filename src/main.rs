pub mod utils;
pub mod config;

use crate::utils::{args_parser::parse_args};

fn main() {
    parse_args();
}
