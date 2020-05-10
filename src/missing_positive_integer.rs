/// Given an unsorted integer array, find the smallest missing positive integer

pub fn minimal_missing_positive(input: Vec<i32>) -> i32 {

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        if nums.len() == 0 {return 1i32;}
        else {
            let mut min: i32 = 1;
            nums.retain(|x| x > &0);
            nums.sort();

            for num in nums.iter() {
                if min < *num {return min;}
                else if min == *num {min += 1;}
            }
            min
        }
    }
}
let result = Solution::first_missing_positive(input);
result
}