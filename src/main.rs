use std::{io, process::exit};


fn main() {
    
    println!("------------------------------------------");
    println!("Personal Expense Tracker");
    println!("------------------------------------------");
    println!("1. Enter Expense Amount\n2. List expenses");

    let mut inp:String = String::new();

    io::stdin()
    .read_line(&mut inp)
    .expect("Error at read line");

    let option:u8 = inp.trim().parse().expect("Error while parsing input");

    match option {
        1 => {

        },
        2 => {

        },
        _ => {
            println!("Invalid Option");
            exit(1);
        }
    }
}

struct Category {
    name: String
}

struct Expense {
    amount: f64,
    description: String,
    category: Category,
    tags: Vec<Tag>
}

struct Tag {
    name: String
}

