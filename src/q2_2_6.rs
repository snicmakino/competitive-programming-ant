mod q2_2_6 {

    pub fn solve(n: usize, wv: Vec<(i32, i32)>, maxw: i32) -> i32 {
        let dfs = Dfs::new(n, wv, maxw);
        dfs.dfs(0, 0, 0)
    }

    struct Dfs {
        n: usize,
        wv: Vec<(i32, i32)>,
        maxw: i32,
    }

    impl Dfs {
        pub fn new(n: usize, wv: Vec<(i32, i32)>, maxw: i32) -> Dfs {
            Dfs { n, wv, maxw }
        }

        pub fn dfs(&self, i: usize, cw: i32, cv: i32) -> i32 {
            if i == self.n {
                return cv;
            }
            let tw = cw + self.wv[i].0;
            let tv = cv + self.wv[i].1;
            if self.maxw < tw {
                return cv;
            }
            std::cmp::max(self.dfs(i + 1, cw, cv), self.dfs(i + 1, tw, tv))
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
