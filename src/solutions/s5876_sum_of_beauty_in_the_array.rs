use super::Solution;

use std::cmp::{max, min};

impl Solution {
    #[allow(dead_code)]
    pub fn sum_of_beauties(numbers: Vec<i32>) -> i32 {
        let mut prefix = numbers[0];
        let mut postfix = vec![numbers[numbers.len() - 1]; numbers.len()];
        let mut result = 0;

        for i in (0..numbers.len() - 1).rev() {
            postfix[i] = min(postfix[i + 1], numbers[i]);
        }

        for i in 1..numbers.len() - 1 {
            if prefix < numbers[i] && numbers[i] < postfix[i + 1] {
                result += 2;
            } else if numbers[i - 1] < numbers[i] && numbers[i] < numbers[i + 1] {
                result += 1;
            }
            prefix = max(prefix, numbers[i]);
        }

        return result;
    }
}
