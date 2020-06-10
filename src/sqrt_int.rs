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
        debug_assert_eq!(test_zero, 0);
    }

    #[test]
    fn test_big_value() {
        let big_value = integer_sqrt(i32::MAX);
        debug_assert_eq!(big_value, 46340);
    }

    #[test]
    fn test_negative_value() {
        let negative_value = integer_sqrt(-7);
        debug_assert_eq!(negative_value, 0);
    }

    #[test]
    fn test_less_than_four() {
        let less_than_four_00 = integer_sqrt(1);
        let less_than_four_01 = integer_sqrt(2);
        let less_than_four_02 = integer_sqrt(3);
        debug_assert_eq!(less_than_four_00, 1);
        debug_assert_eq!(less_than_four_01, 1);
        debug_assert_eq!(less_than_four_02, 1); 
    }

    #[test]
    fn test_middle_case() {
        let middle_case = integer_sqrt(510);
        debug_assert_eq!(middle_case,
        (510f32).sqrt().floor() as i32);
    }
}