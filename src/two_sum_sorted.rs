/// Given an array of integers that is already sorted in ascending order, find
/// two numbers such that they add up to a specific target number.
/// The function twoSum should return indices of the two numbers such that they
/// add up to the target, where index1 must be less than index2.
/// Note:
///
///    Your returned answers (both index1 and index2) are not zero-based.
///    You may assume that each input would have exactly one solution and you
///    may not use the same element twice.
/// 
/// HashMap solution -> low memory consumption, low speed
/// Loop solution -> classical solution, very slow, lower memory consumption
use std::collections::HashMap;

pub fn add_two_numbers_sorted(input_01: Vec<i32>, input_02: i32) -> Vec<i32> {

struct Solution;

impl Solution {
    /// HashMap solution
    pub fn two_sum_hm(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_hm = HashMap::<i32, Vec<usize>>::with_capacity(numbers.len());
        let mut output = Vec::<i32>::with_capacity(2);

        for (index, num) in numbers.iter().enumerate() {
            if num_hm.contains_key(num) {
                let entry = num_hm.get_mut(num);
                if let Some(vector) = entry {
                    vector.push(index);
                }
            }
            else {num_hm.insert(*num, vec![index]);}
        }
        for (index, num) in numbers.iter().enumerate() {
            if output.len() == 2 {break;}
            let diff = target - num;
            match num_hm.get(&diff) {
                Some(vector) => {
                    output.push((index + 1) as i32);
                    output.push((vector[vector.len() - 1] + 1) as i32);
                },
                _ => {},
            }
        }
        output
    }
    /// Loop solution
    pub fn two_sum_loop(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut output = Vec::<i32>::with_capacity(2);
        let max_index = numbers.len() - 1;
        let mut index_0 = 0;
        let mut index_1 = max_index;

        loop {
            if index_0 >= max_index {break;}
            index_1 = max_index;
            loop {
                if index_1 <= index_0 {break;}
                let num_0 = numbers[index_0];
                let num_1 = numbers[index_1];
                let diff = target - num_0;
                if diff < num_0 || diff < 0 {break;}
                if num_1 == diff {
                    output.push((index_0 + 1) as i32);
                    output.push((index_1 + 1) as i32);
                    break;
                }
                index_1 -= 1;
            }
            index_0 += 1;
        }
        output
    }
    /// Fastest solution
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l_step = 0;
        let mut r_step = numbers.len() - 1;

        while(l_step < r_step) {
            let sum = numbers[l_step] + numbers[r_step];
            if sum < target {l_step += 1;}
            else if sum > target {r_step -= 1;}
            else {
                let output = vec![(l_step + 1) as i32, (r_step + 1) as i32];
                return output;
            }
        }
        let output = vec![];
        output
    }
}
let result = Solution::two_sum(input_01, input_02);
result
}