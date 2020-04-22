// #[allow(unused_imports)]
// #[allow(dead_code)]

// mod sum;
// mod sum_array;
// mod sum_array_target;
// mod reverse_integer;
mod integer_max;
mod palindrome;

// use sum::*;
// use sum_array::*;
// use sum_array_target::*;
// use reverse_integer::*;
use integer_max::print_integer_max_values;
use palindrome::*;

fn main() {
    // println!("Hello, world!");
    // input_sum();
    // input_sum_array();
    // sum_pair_to_target_io();
    // reverse_input_i32(-155000);
    print_integer_max_values();
    println!("{}", check_palindrome(121));
}
