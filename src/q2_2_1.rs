mod q2_2_1 {
    pub fn solve(c: Vec<i32>, mut a: i32) -> i32 {
        use std::cmp::min;

        let v = [1, 5, 10, 50, 100, 500];
        let mut ans = 0;
        for i in (1..6).rev() {
            let t = min(a / v[i], c[i]);
            a -= t * v[i];
            ans += t;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_1::*;

    #[test]
    fn test() {
        let c = vec![3, 2, 1, 3, 0, 2];
        let a = 620;
        assert_eq!(solve(c, a), 6);
    }
}
