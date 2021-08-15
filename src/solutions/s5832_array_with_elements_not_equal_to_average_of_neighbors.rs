use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn rearrange_array(mut numbers: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; numbers.len()];

        numbers.sort_unstable();

        let partition = numbers.len() / 2;
        for i in 0..partition {
            result[i * 2 + 1] = numbers[i];
        }
        for i in partition..numbers.len() {
            result[(i - partition) * 2] = numbers[i];
        }

        return result;
    }
}
