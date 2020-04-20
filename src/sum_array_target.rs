use std::io;
use std::collections::HashMap;

#[allow(dead_code)]

pub fn sum_pair_to_target_io() -> Vec<i32> {
    
    struct Solution {
        nums: Vec<i32>,
        target: i32,
    }
    
    impl Solution {
        //pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        pub fn two_sum(&self) -> Vec<i32> {
            //let mut iter_nums = self.nums.iter();
            //let mut intermediate: Vec<i32> = vec![];
            if self.nums.len() < 2 {panic!("Nothing to sum up!")}

            let mut result: Vec<i32> = vec![];
            let mut index: usize = 0;
            let mut check = self.nums[index];

            while result.len() < 2 {
                for (i, item) in self.nums.iter().enumerate() {
                    if i <= index {continue}
                    else {if check + item == self.target {
                        result.push(index as i32);
                        result.push(i as i32);
                        }
                        else {continue};
                    }
                }
                if 0 < index || index < self.nums.len() {
                    index += 1; check = self.nums[index]
                } else {panic!("Vector scanned, no sum pairs!")}
            }
            result
        }
    }
    
    // Processing the input
    let mut str_00 = String::from("[2,7,11,15]");//String::new();
    let mut str_01 = String::from("9");//String::new();
    // Convert any type of input data to Vec<i32>, clean the string from char
    io::stdin().read_line(&mut str_00).ok().expect("read error");

    str_00 = str_00.replace(|c| c == ',' || c == '[' || c == ']', " ");
    let nums: Vec<i32> = str_00
    .split_whitespace()
    .map(|s| s.trim().parse().unwrap())
    .collect();
    // Convert input to i32
    io::stdin().read_line(&mut str_01).ok().expect("read error");
    let target: i32 = str_01.trim().parse().ok().expect("parse error");
    // Init Solution and get the possible sum items, if no solutions - panic
    let initial_vec = Solution {nums, target};
    let result = initial_vec.two_sum();
    println!("{:?}", result);
    result
}

pub fn sum_pair_to_traget_hash() {
    struct Solution {
        nums: Vec<i32>,
        target: i32,
    }
    
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            //  Hash map for fast calls instead of iterators
            let mut checked = HashMap::new();
            // Convert Vec to iterator to scan values
            let nums_iter = nums.iter().enumerate();
            // Scan the input Vec for values
            // if target - value = some_value in Vec the solution is found
            for (i, item) in  nums_iter {
                let test_value = target - item;
                if checked.contains_key(&test_value) {
                    // Return must be Vec<i32>
                    return vec![checked[&test_value] as i32, i as i32];
                }
                checked.insert(item, i);
            }
            unreachable!()
        }
    }
}