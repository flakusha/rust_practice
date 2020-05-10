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
// mod sorted_list_without_dupl_len;
// mod remove_element_from_list;
// mod detect_insertion;
// mod string_to_int;
// mod vector_plus_one;
// mod missing_positive_integer;
mod histogram_largest_rectangle_area;

// use sum::*;
// use sum_array::*;
// use sum_array_target::*;
// use reverse_integer::*;
// use max_numbers::print_integer_max_values;
// use palindrome::*;
// use two_sum_linked_list::*;
// use longest_prefix::*;
// use sorted_list_without_dupl_len::*;
// use remove_element_from_list::*;
// use detect_insertion::*;
// use string_to_int::*;
// use vector_plus_one::*;
// use missing_positive_integer::*;
use histogram_largest_rectangle_area::*;

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

    // sorted_list_without_dupl_len
    // let mut ordered_vector = vec![1, 1, 2, 2, 2, 3, 3, 3, 3];
    // println!("Ordered vector:{0:?}; length: {1}", ordered_vector,
    // ordered_vector.len());
    // let length = remove_duplicates_vec(&mut ordered_vector);
    // println!("Ordered vector:{0:?}; length: {1}; duplicates removed",
    // ordered_vector, length);

    // remove_element_from_list
    // let mut vector: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    // let target: i32 = 2;
    // println!("Vector: {0:?}; Target: {1}", vector, target);
    // let modified_len = remove_target(&mut vector, target);
    // println!("Vector: {0:?}; Length: {1}", vector, modified_len);

    // detect insertion
    // let result = detect_insertion(vec![1,3,5,6], 102);
    // println!("{}", result);

    // Convert string to int
    // println!("{}", string_to_int(String::from("-0 123")));

    // Add 1 to vector
    // println!("{:?}", vector_plus_one(vec![9,9,9,9]))

    // Find minimal missing integer in array
    // minimal_missing_positive(vec![10, 20, 30]);

    //Find largest area in rectangle histogram
    let area = histogram_largest_area(vec![2,1,1,6,7,1,2,2]);
    println!("{}", area);
}