/// Implement int sqrt(int x).
/// Compute and return the square root of x, where x is guaranteed to be a
/// non-negative integer.
/// Since the return type is an integer, the decimal digits are truncated and
/// only the integer part of the result is returned.
/// Using Newton isqrt

pub fn integer_sqrt(input: i32) -> i32 {

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 1 {return 0;}
        else if x < 4 {return 1;}
        else if x == std::i32::MAX {return 46340;}
        else {
            let mut right = x;

            loop {
                right = (right.saturating_add(x / right)) / 2;
                let rightsq = right.saturating_mul(right);
                println!("{}:{}:{}", x, right, rightsq);

                if rightsq <= x {return right;}
            }
        }
    }
}
let output = Solution::my_sqrt(input);
output
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_zero() {
        let test_zero = integer_sqrt(0);
        assert_eq!(test_zero, 0);
    }

    #[test]
    fn test_i32_positive_range() {
        for value in 0..i32::MAX {
            let test_i32_pos_range = integer_sqrt(value);
            assert_eq!(test_i32_pos_range, (value as f32).sqrt().floor() as i32);
        }
    }
}