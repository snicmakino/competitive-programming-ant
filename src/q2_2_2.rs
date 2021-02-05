mod q2_2_2 {
    pub fn solve(n: usize, s: Vec<i32>, t: Vec<i32>) -> i32 {
        let mut tasks = vec![];
        for i in 0..n {
            tasks.push([s[i], t[i]]);
        }
        tasks.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut ans = 0;
        let mut time = 0;

        for task in tasks {
            if time < task[0] {
                ans += 1;
                time = task[1];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_2::*;

    #[test]
    fn test() {
        let n = 5;
        let s = vec![1, 2, 4, 6, 8];
        let t = vec![3, 5, 7, 9, 10];
        assert_eq!(solve(n, s, t), 3);
    }
    #[test]
    fn test2() {
        let n = 5;
        let s = vec![1, 2, 4, 6, 8];
        let t = vec![7, 3, 7, 9, 10];
        assert_eq!(solve(n, s, t), 3);
    }
}
