mod q2_2_4 {

    pub fn solve(n: usize, r: i32, mut x: Vec<i32>) -> i32 {
        x.sort();
        let mut ans = 0;
        let mut min = x[0];

        for i in 1..n {
            if min + r <= x[i] {
                ans += 1;
                min = x[i];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_4::*;

    #[test]
    fn test() {
        let n = 6;
        let r = 10;
        let x = vec![1, 7, 15, 20, 30, 50];
        assert_eq!(solve(n, r, x), 3);
    }
    #[test]
    fn test2() {
        let n = 3;
        let r = 0;
        let x = vec![10, 20, 20];
        assert_eq!(solve(n, r, x), 2);
    }
    #[test]
    fn test3() {
        let n = 7;
        let r = 10;
        let x = vec![70, 30, 1, 7, 15, 20, 50];
        assert_eq!(solve(n, r, x), 4);
    }
}
