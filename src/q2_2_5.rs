mod q2_2_5 {
    pub fn solve(n: usize, mut l: Vec<i32>) -> i32 {
        l.sort();
        let mut cost = 0;

        for i in 0..n - 1 {
            if i + 2 > n - 1 || l[i] < l[i + 2] {
                let sum = l[i] + l[i + 1];
                cost += sum;
                l[i + 1] = sum;
            } else {
                let sum = l[i + 1] + l[i + 2];
                cost += sum;
                l[i + 2] = sum;
                l[i + 1] = l[i];
            }
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_5::*;

    #[test]
    fn test() {
        let n = 3;
        let l = vec![8, 5, 8];
        assert_eq!(solve(n, l), 34);
    }
}
