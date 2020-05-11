/// Given two binary strings, return their sum (also a binary string)
/// The input strings are both non-empty and contains only characters 1 or 0

pub fn binary_string_sum(input_00: String, input_01: String) -> String{

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // Compare, place longer string first
        let (a, b) = {match a.len() > b.len() {
                true => (a, b),
                _ => (b, a)
            }
        };

        let difference = a.len() - b.len();
        // So called carry, boolean takes less space
        let mut add = '0';
        // Avoid reallocation of output during the algorithm
        let mut out = String::with_capacity(a.len() + 1);
        // Reversed Vector<Bool> representation for string
        let bool_00: Vec<bool> = a.chars().rev()
        .map(|x| {if x == '0' {false} else {true}}).collect();
        let mut bool_01: Vec<bool> = b.chars().rev()
        .map(|x| {if x == '0' {false} else {true}}).collect();
        // Make len of bools identical
        let index_max = bool_00.len() - 1;
        let mut index = 0;
        (0..difference).for_each(|_| bool_01.push(false));

        loop {
            if index > index_max && add == '0' {break;}
            else if index > index_max && add == '1' {out.push(add); break;}
            else {
                // 0 & 0
                if !bool_00[index] && !bool_01[index] {
                    out.push(add);
                    add = '0';
                }
                // 1 & 1
                else if bool_00[index] && bool_01[index] {
                    out.push(add);
                    add = '1';
                }
                // 0 & 1 || 1 & 0
                else {
                    if add == '0' {out.push('1');}
                    else {out.push('0');}
                }
            }
            index += 1;
        }
        out = out.chars().rev().collect();
        out
    }
}    
let result = Solution::add_binary(input_00, input_01);
result
}