/// Algorithm returns Vec len without duplicates, original Vec is modified

pub fn remove_duplicates_vec(input: &mut Vec<i32>) -> i32 {
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // 0 or 1 items -> just return the len of Vec
        if nums.len() == 0 || nums.len() == 1 {return nums.len() as i32;}
        
        let mut pos = 0;

        loop {
            // Loop until solution is found
            // Start from the beginning of the Vec
            // Increment pos only if no duplicates detected
            if pos > 0 && nums[pos] == nums[pos - 1] {nums.remove(pos);}
            else {pos += 1};
            if pos >= nums.len() {return nums.len() as i32;}
        }
    }

    pub fn remove_duplicates_01(nums: &mut Vec<i32>) -> i32 {
        // 0 or 1 items -> just return the len of Vec
        if nums.len() == 0 || nums.len() == 1 {return nums.len() as i32;}
        
        let mut pos = 0;
        loop {
            // Loop infinetely, removing every duplicating item
            // until reached the len of Vec
            if pos >= nums.len() {return nums.len() as i32;}
            // Start from the beginning of the Vec, buffer the item
            let buf_item = nums[0];
            // Remove repeating
            nums.retain(|&item| item != buf_item);
            // Write filtered number to the end, repeat cycle
            nums.push(buf_item);
            pos += 1;
        }
    }
}

let result = Solution::remove_duplicates(input);
result

}