/// Given the sorted Vec<i32> detect where is insertion, or where would it be
/// if it's not in array

pub fn detect_insertion(input: Vec<i32>, value: i32) -> i32 {

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {return 0i32;}
        else if nums[0] > target {return 0i32;}
        else if nums[nums.len() - 1] < target {return nums.len() as i32;}
        else {
            let mut pos = 0;
            loop {
                if pos >= nums.len() {return nums.len() as i32;}
                else if nums[pos] == target {return pos as i32;}
                else if pos > 0 && nums[pos - 1] < target && nums[pos] > target
                && nums[pos] != target {return pos as i32;}
                pos += 1;
            }
        }
    }
}

let result = Solution::search_insert(input, value);
result
}