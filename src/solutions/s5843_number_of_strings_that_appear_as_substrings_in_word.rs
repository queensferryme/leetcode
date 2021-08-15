use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut result = 0;

        for pattern in patterns {
            if word.find(pattern.as_str()).is_some() {
                result += 1;
            }
        }

        return result;
    }
}
