mod q2_1_1_loop {
    pub fn solve(n: usize, a: Vec<i32>, k: i32) -> bool {
        let mut stack = vec![Node { i: 0, sum: 0 }];
        while !stack.is_empty() {
            let node: Node = stack.pop().unwrap();
            if node.sum == k {
                return true;
            }

            if node.i == n {
                continue;
            }

            for b in [0, 1].iter() {
                stack.push(Node {
                    i: node.i + 1,
                    sum: node.sum + a[node.i] * b,
                });
            }
        }
        false
    }

    struct Node {
        i: usize,
        sum: i32,
    }
}

#[cfg(test)]
mod tests {
    use super::q2_1_1_loop::*;

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
