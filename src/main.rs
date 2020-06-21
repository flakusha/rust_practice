// #[allow(unused_imports)]
// #[allow(dead_code)]

// mod sum;
// mod sum_array;
// mod sum_array_target;
// mod reverse_integer;
// mod max_numbers;
// mod palindrome;

// mod longest_prefix;
// mod sorted_list_without_dupl_len;
// mod remove_element_from_list;
// mod detect_insertion;
// mod string_to_int;
// mod vector_plus_one;
// mod missing_positive_integer;
// mod histogram_largest_rectangle_area;
// mod histogram_largest_rectangle_area_simple;
// mod binary_string_sum;
// mod two_sum_sorted;
// mod sqrt_int;
// mod merge_sorted_arrays;
mod pascal_triangle;

// use sum::*;
// use sum_array::*;
// use sum_array_target::*;
// use reverse_integer::*;
// use max_numbers::print_integer_max_values;
// use palindrome::*;
// use longest_prefix::*;
// use sorted_list_without_dupl_len::*;
// use remove_element_from_list::*;
// use detect_insertion::*;
// use string_to_int::*;
// use vector_plus_one::*;
// use missing_positive_integer::*;
// use histogram_largest_rectangle_area::*;
// use histogram_largest_rectangle_area_simple::*;
// use binary_string_sum::*;
// use two_sum_sorted::*;
// use sqrt_int::*;

// Use these modules together
// mod two_sum_linked_list;
// mod remove_duplicates_linked_list;
// use two_sum_linked_list::*;
// use remove_duplicates_linked_list::*;

// use merge_sorted_arrays::*;
use pascal_triangle::*;

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

    // Find largest area in rectangle histogram
    // let area = histogram_largest_area(vec![2,1,1,6,7,1,2,2]);
    // println!("{}", area);

    // Sum up two binary strings
    // let two_binary_string_sum = binary_string_sum(
    //     String::from("111"),
    //     String::from("11")
    // );
    // println!("{:?}", two_binary_string_sum);

    // Sum up two sorted Vec<i32>
    // let two_sum_sorted_vec: Vec<i32> = add_two_numbers_sorted(
    //     vec![4,4,6,6,6,8,9,56,90,90],
    //     146);
    // println!("{:?}", two_sum_sorted_vec);

    // Return integer part of sqrt
    // let sqrt_int_result = integer_sqrt(i32::MAX - 1);
    // println!("{}", sqrt_int_result);

    // Remove duplicates from linked list
    // let no_dupl_ll =
    // rem_dupl_ll(vec![1,1,1,2,2,3,3,3,4,4,4,4,5,5,5,5,5,5,6,6,7,8]);

    // Merge sorted arrays
    // merge_two_arrays(&mut vec![1,2,3], &mut vec![4,5,6]);
    // merge_two_arrays(&mut vec![4,5,6], &mut vec![1,2,3]);
    // merge_two_arrays(&mut vec![0,0,0,1,2,3,4],
    // &mut vec![0,0,0,0,3,4,5,6]);

    // Calculate the Pascal triangle
    let rows_num = 34;
    let pascal_triangle = calculate_pascal_triangle(rows_num);
    for vector in pascal_triangle {
        for item in vector {
            print!(" {} ", item);
        }
        println!();
        //println!("{:^width$?}", vector, width = (rows_num as usize) * 2 + 1);
    }
}