use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn get_lucky(string: String, k: i32) -> i32 {
        let mut result = 0;

        for ch in string.chars() {
            result += get_sum_of_digits(ch as i32 - 'a' as i32 + 1);
        }

        for _ in 1..k {
            result = get_sum_of_digits(result);
        }

        return result;
    }
}

fn get_sum_of_digits(mut number: i32) -> i32 {
    let mut sum = 0;

    while number != 0 {
        sum += number % 10;
        number /= 10;
    }

    return sum;
}
