
pub fn reverse_input_i32(main_input:i32) -> i32 {
// No identation, because it's for the site

struct Solution {
    x: i32,
}

impl Solution {
    pub fn reverse(x:i32) -> i32 {
        
        // Struct to identify input
        struct Input {
            num: i32,
            positive: bool,
        }

        let input =
        if x < 0 {Input {num: -x, positive: false}}
        else {Input {num: x, positive: true}};
        
        let int_str = input.num.to_string();
        let conv = int_str.chars().rev().collect::<String>();
        let mut out: i32 = conv.parse().unwrap_or(0);
        out = if input.positive {out} else {-out};
        out
        }

    pub fn reverse_self(&self) -> i32 {
        // Struct to identify input
        struct Input {
            num: i32,
            positive: bool,
        }

        let input =
        if self.x < 0 {Input {num: -self.x, positive: false}}
        else {Input {num: self.x, positive: true}};
        
        let int_str = input.num.to_string();
        let conv = int_str.chars().rev().collect::<String>();
        let mut out: i32 = conv.parse().unwrap_or(0);
        out = if input.positive {out} else {-out};
        out
        }
    }

let output: i32 = Solution{x: main_input}.reverse_self();
println!("{}", output);
output
}