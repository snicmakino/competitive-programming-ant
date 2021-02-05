mod q2_2_1 {
    pub fn solve(c: Vec<i32>, a: i32) -> i32 {
        let coins = [
            [500, c[5]],
            [100, c[4]],
            [50, c[3]],
            [10, c[2]],
            [5, c[1]],
            [1, c[0]],
        ];

        let mut num = 0;
        let mut sum = 0;
        for coin in coins.iter() {
            let coin_price = coin[0];
            let coin_num = coin[1];
            for i in (0..coin_num + 1).rev() {
                if i * coin_price + sum <= a {
                    num += i;
                    sum += i * coin_price;
                    break;
                }
            }
        }
        return num;
    }
}

#[cfg(test)]
mod tests {
    use super::q2_2_1::*;

    #[test]
    fn test() {
        let c = vec![3, 2, 1, 3, 0, 2];
        let a = 620;
        assert_eq!(solve(c, a), 6);
    }
}
