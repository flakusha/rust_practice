/// Algorithm takes two arrays from keyboard and sums them

use std::io;

pub fn input_sum_array() {
    // init array size
    let mut array_size = String::new();
    // init input
    let mut array_members = String::new();

    io::stdin().read_line(&mut array_size).ok().expect("read error");
    let mut array_size_num: i32 = array_size.trim().parse().ok().expect("parse error");

    io::stdin().read_line(&mut array_members).ok().expect("read error");
    let mut array_members_num: Vec<i32> = array_members.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    
    let mut array_sum_result:i32 = array_members_num.into_iter().sum();

    println!("{}", array_sum_result);

}