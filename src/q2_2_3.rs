mod q2_2_3 {
    use std::collections::VecDeque;

    pub fn solve(n: usize, s: &str) -> String {
        let mut s0 = s.chars().collect::<VecDeque<char>>();

        let mut ans: String = String::new();
        for _ in 0..n {
            let ts = s0.iter().collect::<String>();
            let tr = s0.iter().rev().collect::<String>();
            ans.push(if ts < tr {
                s0.pop_front().unwrap()
            } else {
                s0.pop_back().unwrap()
            });
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_3::*;

    #[test]
    fn test() {
        let n = 6;
        let s = "ACDBCB";
        assert_eq!(solve(n, s), "ABCBCD");
    }
}
