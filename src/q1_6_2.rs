mod q1_6_2 {
    use std::cmp;
    pub fn solve(l: i32, n: usize, x: Vec<i32>) -> [i32; 2] {
        let mut min = 0;
        let mut max = 0;
        for i in 0..n {
            min = cmp::max(min, cmp::min(x[i], l - x[i]));
            max = cmp::max(max, cmp::max(x[i], l - x[i]));
        }
        [min, max]
    }
}
#[cfg(test)]
mod tests {
    use super::q1_6_2::*;

    #[test]
    fn test() {
        let l = 10;
        let n = 3;
        let x = vec![2, 6, 7];
        assert_eq!(solve(l, n, x), [4, 8]);
    }
    #[test]
    fn test2() {
        let l = 214;
        let n = 7;
        let x = vec![11, 12, 7, 13, 176, 23, 191];
        assert_eq!(solve(l, n, x), [38, 207]);
    }
}
