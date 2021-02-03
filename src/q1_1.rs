mod q1_1 {
    pub fn solve(n: usize, m: i32, mut k: Vec<i32>) -> bool {
        for a in 0..n {
            for b in 0..n {
                for c in 0..n {
                    for d in 0..n {
                        if k[a] + k[b] + k[c] + k[d] == m {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::q1_1::*;

    #[test]
    fn test() {
        let n = 3;
        let m = 10;
        let k = vec![1, 3, 5];
        assert_eq!(solve(n, m, k), true);
    }
    #[test]
    fn test2() {
        let n = 3;
        let m = 9;
        let k = vec![1, 3, 5];
        assert_eq!(solve(n, m, k), false);
    }
}
