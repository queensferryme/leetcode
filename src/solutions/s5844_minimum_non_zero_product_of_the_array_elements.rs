use super::Solution;

impl Solution {
    const MOD: i64 = 1000000007;

    #[allow(dead_code)]
    pub fn min_non_zero_product(p: i32) -> i32 {
        let m = ((1i64 << p) - 2) % Solution::MOD;
        let m_power = quick_power(m, (1i64 << (p - 1)) - 1);
        let result = m_power * (m + 1) % Solution::MOD;

        return result as i32;
    }
}

fn quick_power(mut base: i64, mut expo: i64) -> i64 {
    let mut result = 1i64;
    while expo > 0 {
        if (expo & 1) != 0 {
            result = result * base % Solution::MOD;
        }
        base = base * base % Solution::MOD;
        expo >>= 1;
    }
    return result;
}
