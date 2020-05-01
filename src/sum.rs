/// Module sums two inputs

use std::io;

pub fn input_sum() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    let mut num_01 : i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let mut num_02 : i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // print the sum 
    println!("{}", num_01 + num_02);
}

