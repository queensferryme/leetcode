use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;

        for operation in operations {
            match operation.as_str() {
                "++X" | "X++" => result += 1,
                "--X" | "X--" => result -= 1,
                _ => panic!(),
            }
        }

        return result;
    }
}
