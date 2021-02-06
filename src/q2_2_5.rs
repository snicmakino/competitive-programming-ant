mod q2_2_5 {
    use std::collections::VecDeque;

    pub fn solve(_n: usize, mut l: Vec<i32>) -> i32 {
        l.sort();
        let mut board = VecDeque::from(l);
        let mut cost = 0;

        while board.len() > 2 {
            let n1 = board.pop_front().unwrap();
            let n2 = board.pop_front().unwrap();
            let n3 = board.pop_front().unwrap();
            if n1 < n3 {
                cost += n1 + n2;
                board.push_front(n3);
                board.push_front(n1 + n2);
            } else {
                cost += n2 + n3;
                board.push_front(n1);
                board.push_front(n2 + n3);
            }
        }
        cost += board[0] + board[1];
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
