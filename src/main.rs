// #[allow(unused_imports)]
// #[allow(dead_code)]

// mod sum;
// mod sum_array;
// mod sum_array_target;
// mod reverse_integer;
// mod max_numbers;
// mod palindrome;
// mod two_sum_linked_list;
// mod longest_prefix;
mod sorted_list_without_dupl_len;

// use sum::*;
// use sum_array::*;
// use sum_array_target::*;
// use reverse_integer::*;
// use max_numbers::print_integer_max_values;
// use palindrome::*;
// use two_sum_linked_list::*;
// use longest_prefix::*;
use sorted_list_without_dupl_len::*;

fn main() {
    // println!("Hello, world!");
    // input_sum();
    // input_sum_array();
    // sum_pair_to_target_io();
    // reverse_input_i32(-155000);
    // print_integer_max_values();
    // println!("{}", check_palindrome(121));
    // add_two_numbers_linked_list(vec![1,2,3], vec![1,2,3]);
    // common_prefix_example();
    let mut ordered_vector = vec![1, 1, 2, 2, 2, 3, 3, 3, 3];
    println!("Ordered vector:{0:?}; length: {1}", ordered_vector,
    ordered_vector.len());
    let length = remove_duplicates_vec(&mut ordered_vector);
    println!("Ordered vector:{0:?}; length: {1}; duplicates removed",
    ordered_vector, length);
}
