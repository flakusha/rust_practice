/// Given a non-negative integer numRows, generate the first numRows of Pascal's 
/// triangle.
// use std::sync::{Arc, Mutex};
// use std::thread;

pub fn calculate_pascal_triangle(input: i32) -> Vec<Vec<i32>> {

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut nrs = num_rows;
        if nrs < 0 {nrs *= -1;}
        let nr = nrs as usize; 
        if nr == 0 {return vec![];}
        else if nr == 1 {return vec![vec![1]];}
        else if nr == 2 {return vec![vec![1], vec![1,1]];}

        let mut pascal_triangle = Vec::<Vec<i32>>::new();
        for line in 1..= nr {
            let mut pascal_line = Vec::<i32>::with_capacity(line as usize);
            let mut item = 1;
            for index in 1..= line {
                pascal_line.push(item as i32);
                item = item * (line - index) / index;
            }
            pascal_triangle.push(pascal_line);
        }
        return pascal_triangle;
    }
}
let output = Solution::generate(input);
output
}