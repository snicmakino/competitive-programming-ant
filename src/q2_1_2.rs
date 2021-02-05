mod q2_1_2 {
    pub fn solve(n: i32, m: i32, lake: Vec<Vec<&str>>) -> i32 {
        let mut result = 0;
        let mut dfs = Dfs::new(n, m, lake);

        for i in 0..n {
            for j in 0..m {
                if "W" == dfs.lake[i as usize][j as usize] {
                    dfs.dfs(i, j);
                    result += 1;
                }
            }
        }
        result
    }

    struct Dfs<'a> {
        nsize: i32,
        msize: i32,
        lake: Vec<Vec<&'a str>>,
    }

    impl Dfs<'_> {
        pub fn new(n: i32, m: i32, lake: Vec<Vec<&str>>) -> Dfs {
            Dfs {
                nsize: n,
                msize: m,
                lake,
            }
        }

        pub fn dfs(&mut self, x: i32, y: i32) {
            self.lake[x as usize][y as usize] = ".";
            for i in -1..2 {
                for j in -1..2 {
                    let nx = x + i;
                    let ny = y + j;
                    if 0 <= nx
                        && nx < self.nsize
                        && 0 <= ny
                        && ny < self.msize
                        && self.lake[nx as usize][ny as usize] == "W"
                    {
                        self.dfs(nx, ny);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::q2_1_2::*;

    #[test]
    fn test() {
        let n = 10;
        let m = 12;
        let lake = vec![
            vec!["W", ".", ".", ".", ".", ".", ".", ".", ".", "W", "W", "."],
            vec![".", "W", "W", "W", ".", ".", ".", ".", ".", "W", "W", "W"],
            vec![".", ".", ".", ".", "W", "W", ".", ".", ".", "W", "W", "."],
            vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "W", "W", "."],
            vec![".", ".", ".", ".", ".", ".", ".", ".", ".", "W", ".", "."],
            vec![".", ".", "W", ".", ".", ".", ".", ".", ".", "W", ".", "."],
            vec![".", "W", ".", "W", ".", ".", ".", ".", ".", "W", "W", "."],
            vec!["W", ".", "W", ".", "W", ".", ".", ".", ".", ".", "W", "."],
            vec![".", "W", ".", "W", ".", ".", ".", ".", ".", ".", "W", "."],
            vec![".", ".", "W", ".", ".", ".", ".", ".", ".", ".", "W", "."],
        ];
        assert_eq!(solve(n, m, lake), 3);
    }
}
