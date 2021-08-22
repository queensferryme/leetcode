use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_quadruplets(numbers: Vec<i32>) -> i32 {
        let mut result = 0;

        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                for k in j + 1..numbers.len() {
                    let sum = numbers[i] + numbers[j] + numbers[k];
                    result += numbers.iter().skip(k + 1).filter(|&n| *n == sum).count();
                }
            }
        }

        return result as i32;
    }
}
