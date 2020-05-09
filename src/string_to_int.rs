/// Algorithm converts input string into integer in case string starts with
/// minus sign or numeric char

//use std::num::IntErrorKind;

pub fn string_to_int(input: String) -> i32 {

struct Solution;

impl Solution {
    pub fn my_atoi(string: String) -> i32 {
        if string.len() == 0 {return 0i32;}

        // Allocate string with additional capacity to avoid reallocation
        let mut final_str = String::with_capacity(100);
        let mut char_is_not_number = false;
        let mut char_sign_detected = false;
        let mut char_num_counter: u32 = 0;
        let mut char_sign_counter: u8 = 0;

        for sym in string.chars() {
            if char_is_not_number {break;}
            if char_sign_detected && char_sign_counter > 1 {break;}
            if sym == ' ' && (char_sign_counter > 0 || char_num_counter > 0)
            {break;}
            if (sym == '+' || sym == '-') && char_num_counter > 0 {break;}
            // Match the symbol with numbers and signs, if there is more 
            match sym {
                ' ' => {continue;}
                '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' =>
                {final_str.push(sym); char_num_counter += 1},
                '+' => {
                    if char_sign_counter == 0 {
                        char_sign_detected = true;
                        char_sign_counter += 1;
                    }
                    else {
                        char_sign_counter += 1;
                    }
                },
                '-' => {
                    if char_sign_counter == 0 {
                        char_sign_detected = true;
                        char_sign_counter += 1;
                        final_str.push(sym);
                    }
                    else {
                        char_sign_counter += 1;
                    }
                },
                _ => {char_is_not_number = true},
            }
        }
        
        if final_str.len() == 0 {return 0i32;}
        
        let out: i32 = match final_str.parse() {
            Ok(unwrapped) => unwrapped,
            Err(e) => {
                println!("{:?}, {}", e, e.to_string());
                let empty =
                String::from("cannot parse integer from empty string");
                let overflow =
                String::from("number too large to fit in target type");
                let underflow =
                String::from("number too small to fit in target type");

                if e.to_string() == empty {return 0i32;}
                else if e.to_string() == overflow {return i32::max_value();}
                else if e.to_string() == underflow {return i32::min_value();}
                else {return 0i32;}
            }
        };
        out
    }
}

let result = Solution::my_atoi(input);
result
}