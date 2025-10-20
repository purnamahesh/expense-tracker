use std::io;


pub fn read_sting(prompt: &str) -> String{

    println!("{}", prompt);

    let mut inp:String = String::new();

    io::stdin()
    .read_line(&mut inp)
    .expect("Error at read line");

    return inp;
}

pub fn read_u8(prompt: &str) -> u8{

    let inp:String = read_sting(prompt);

    let result:u8 = inp.trim().parse().expect("Error while parsing input");

    return result;
}

pub fn read_f64(prompt: &str) -> f64{

    let inp:String = read_sting(prompt);

    let result:f64 = inp.trim().parse().expect("Error while parsing input");

    return result;
}