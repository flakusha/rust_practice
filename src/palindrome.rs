/// Algorhitm checs that two integers are palindromes
/// Reverse:
/// Take number.len() - 1 and sum with number.len() - 2
/// Palindrome:
/// true if not negative and num_orig = num_rev

pub fn check_palindrome(main_input: i32) -> bool {

struct Solution {x:i32}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        const RAD: i32 = 10;
        let mut input: i32 = x;
        let original = x;

        if input < 0 {false}

        else {

            let mut rev: i32 = 0;
            // Reversed version of input
            while input != 0 {
                rev = rev * RAD + input % RAD;
                input /= RAD;
            }
            if original == rev {true}
            else {false}
        }
    }

    pub fn is_palindrome_self(&self) -> bool {
        const RAD: i32 = 10;
        let mut input: i32 = self.x;
        let original: i32 = self.x;

        if input < 0 {return false}

        else {

            let mut rev: i32 = 0;
            // Reversed version of input
            while input != 0 {
                rev = rev * RAD + input % RAD;
                input /= RAD;
            }
        if original == rev {return true}
        else {return false}
        }
    }
}

Solution{x: main_input}.is_palindrome_self()

}