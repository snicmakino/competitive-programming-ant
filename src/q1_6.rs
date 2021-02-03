mod q1_6 {
    use std::cmp::Reverse;
    pub fn solve(n: usize, mut a: Vec<i32>) -> i32 {
        a.sort_by_key(|w| Reverse(*w));

        for i in 0..n - 2 {
            if a[i] < a[i + 1] + a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }

    pub fn is_make_trianble(i: i32, j: i32, k: i32) -> bool {
        let mut v = [i, j, k];
        v.sort();
        v[2] < v[0] + v[1]
    }
}

#[cfg(test)]
mod tests {
    use super::q1_6::*;

    #[test]
    fn test() {
        let n = 5;
        let a = vec![2, 3, 4, 5, 10];
        assert_eq!(solve(n, a), 12);
    }
    #[test]
    fn test2() {
        let n = 4;
        let a = vec![4, 5, 10, 20];
        assert_eq!(solve(n, a), 0);
    }
}
