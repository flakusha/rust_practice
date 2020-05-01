/// Algorhitm searches for the longest common prefix in input vector of strings
/// Returns "" if there is no answer

pub fn common_prefix_example() {

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        let words = &strs;
        let words_len = words.into_iter().map(|word| word.len() as i32)
        .collect::<Vec<i32>>();
        let words_len_min = words_len.iter().min().unwrap_or(&0);

        // Avoid reallocation during the loop, create strings with min capacity
        let mut prefix = String::with_capacity(*words_len_min as usize);
        let mut temp = String::with_capacity(*words_len_min as usize);

        let prefix = match *words_len_min {
            // Empty String, so can return ""
            0 => String::from(""),
            _ => {
                let mut iteration = 0;
                let mut loop_prefix = 
                String::with_capacity(*words_len_min as usize);
                let mut finished_search = false;
                
                // Breaks if finished_search == true
                while temp.len() != *words_len_min as usize {
                    temp.push(words[0].chars().nth(iteration).unwrap());

                    for word in words.iter() {
                        if word.starts_with(&temp) {}
                        else {finished_search = true;}
                    }
                    if finished_search {break;}
                    else {
                        loop_prefix = temp.clone();
                        iteration += 1;
                    }
                }
                loop_prefix
            },
        };
        prefix
    }
}

let result = Solution::longest_common_prefix(
    vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]);
println!("{}", result);

}