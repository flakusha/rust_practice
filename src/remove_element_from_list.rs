/// Algorithm removes all the instances of given i32 in Vec<i32>

pub fn remove_target(input_vector: &mut Vec<i32>, target: i32) -> i32 {

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut pos = 0;
        if nums.len() == 0 {return 0i32;}
        loop {
            if nums[pos] == val {nums.remove(pos);}
            else {pos += 1;}
            if pos >= nums.len() {return nums.len() as i32;}
        }
    }
}

let result = Solution::remove_element(input_vector, target);
result

}