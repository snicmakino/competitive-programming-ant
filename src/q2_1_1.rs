mod q2_1_1 {
    pub fn solve(n: usize, a: Vec<i32>, k: i32) -> bool {
        dfs(0, 0, a, k)
    }

    fn dfs(index: usize, sum: i32, vec: Vec<i32>, target: i32) -> bool {
        if sum == target {
            return true;
        }
        if vec.len() > index {
            if dfs(index + 1, sum, vec.clone(), target) {
                return true;
            }
            if dfs(index + 1, sum + vec[index], vec.clone(), target) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::q2_1_1::*;

    #[test]
    fn test() {
        let n = 4;
        let a = vec![1, 2, 4, 7];
        let k = 13;
        assert_eq!(solve(n, a, k), true);
    }
    #[test]
    fn test2() {
        let n = 4;
        let a = vec![1, 2, 4, 7];
        let k = 15;
        assert_eq!(solve(n, a, k), false);
    }
}
