use std::io;
use std::any::type_name;

fn calculate_sums(vector_of_nums: Vec<i32>) -> Vec<i32> {
    vec![0i32, 1i32]
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub fn sum_array_target() {

    struct Solution {
        nums: Vec<i32>,
        target: i32,
    }
    
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            
            

            let mut initial_vec: Solution = Solution{nums, target};
            
            for num in initial_vec.nums {
                if num == target {vec![num]}}
                //else {calculate_sums(nums)}
        }
    }
}