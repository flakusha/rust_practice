/// Given a non-empty array of digits representing a non-negative integer,
/// plus one to the integer.

pub fn vector_plus_one(input: Vec<i32>) -> Vec<i32> {

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        if digits.len() == 0 {digits.insert(0, 1); return digits;}
        
        let mut add_val = true;
        let mut index = digits.len() - 1;

        loop {
            if add_val == false {break;}
            else if add_val == true && digits[index] + 1 <= 9 {
                digits[index] = digits[index] + 1; add_val = false;
                if index != 0 {index -= 1;}
            }
            else if add_val == true && digits[index] + 1 > 9 {
                digits[index] = digits[index] + 1 - 10; add_val = true;
                if index != 0 {index -= 1;}
                else {digits.insert(0, 1); add_val = false;}
            }
            else if add_val == false && digits[index] == 0 {
               digits.remove(index);
            }
        }
        digits
    }
}
let result = Solution::plus_one(input);
result
}