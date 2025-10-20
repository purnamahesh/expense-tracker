use std::env::args;

pub mod utils;

use crate::utils::get_and_prcess_input;

fn main() {

    let args:Vec<String> = args().collect();

    for arg in args {
        println!("{}", arg)
    }

    // get_and_prcess_input();
    
}

