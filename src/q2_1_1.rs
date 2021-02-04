mod q2_1_1 {
    pub fn solve(n: usize, a: Vec<i32>, k: i32) -> bool {
        let dfs = Dfs::new(n, a, k);
        dfs.dfs(0, 0)
    }

    struct Dfs {
        n: usize,
        list: Vec<i32>,
        target: i32,
    }

    impl Dfs {
        pub fn new(n: usize, list: Vec<i32>, target: i32) -> Dfs {
            Dfs { n, list, target }
        }

        pub fn dfs(&self, i: usize, sum: i32) -> bool {
            if i == self.n {
                return self.target == sum;
            }
            if self.dfs(i + 1, sum) {
                return true;
            }
            if self.dfs(i + 1, sum + self.list[i]) {
                return true;
            }
            false
        }
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
