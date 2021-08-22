use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_different_binary_string(numbers: Vec<String>) -> String {
        let n = numbers[0].len();
        let mut numbers = numbers
            .iter()
            .map(|number| i32::from_str_radix(number.as_str(), 2).ok().unwrap())
            .collect::<Vec<i32>>();
        numbers.sort_unstable();
        numbers.dedup();

        for (a, b) in numbers.iter().zip(0..n) {
            if *a != (b as i32) {
                return format!("{:0n$b}", b, n = n);
            }
        }

        return format!("{:0n$b}", n, n = n);
    }
}
