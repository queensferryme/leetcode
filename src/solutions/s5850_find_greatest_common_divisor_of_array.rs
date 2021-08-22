use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_gcd(numbers: Vec<i32>) -> i32 {
        let maximum = numbers.iter().max().unwrap();
        let minimum = numbers.iter().min().unwrap();

        return gcd(*maximum, *minimum);
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
