mod q2_2_6 {
    use std::collections::HashMap;

    pub fn solve(n: usize, wv: Vec<(i32, i32)>, maxw: i32) -> i32 {
        let mut dfs = Dfs::new(n, wv);
        dfs.dfs(0, maxw)
    }

    struct Dfs {
        n: usize,
        wv: Vec<(i32, i32)>,
        memo: HashMap<(usize, i32), i32>,
    }

    impl Dfs {
        pub fn new(n: usize, wv: Vec<(i32, i32)>) -> Dfs {
            Dfs {
                n,
                wv,
                memo: HashMap::new(),
            }
        }

        pub fn dfs(&mut self, i: usize, w: i32) -> i32 {
            if self.memo.contains_key(&(i, w)) {
                return *self.memo.get(&(i, w)).unwrap();
            }

            if i == self.n {
                return 0;
            }
            let cw = self.wv[i].0;
            let cv = self.wv[i].1;
            if w < cw {
                return 0;
            }

            let max = std::cmp::max(self.dfs(i + 1, w), self.dfs(i + 1, w - cw) + cv);
            self.memo.insert((i, w), max);
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_6::*;

    #[test]
    fn test() {
        let n = 4;
        let wv = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
        let w = 5;
        assert_eq!(solve(n, wv, w), 7);
    }
}
