mod q1_6;

fn main() {
    println!("Hello, world!");
}

// pub fn solve(n: usize, a: &[i32]) -> i32 {
//     let mut result = 0;

//     for i in 0..n {
//         for j in i + 1..n {
//             for k in j + 1..n {
//                 if is_make_trianble(a[i], a[j], a[k]) {
//                     if result < a[i] + a[j] + a[k] {
//                         result = a[i] + a[j] + a[k];
//                     }
//                 }
//             }
//         }
//     }
//     result
// }

// pub fn is_make_trianble(i: i32, j: i32, k: i32) -> bool {
//     let mut v = [i, j, k];
//     v.sort();
//     v[2] < v[0] + v[1]
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         let n = 5;
//         let a = [2, 3, 4, 5, 10];
//         assert_eq!(solve(n, &a), 12);
//     }
//     #[test]
//     fn test2() {
//         let n = 4;
//         let a = [4, 5, 10, 20];
//         assert_eq!(solve(n, &a), 0);
//     }
// }

// macro_rules! input {
//     (source = $s:expr, $($r:tt)*) => {
//         let mut iter = $s.split_whitespace();
//         input_inner!{iter, $($r)*}
//     };
//     ($($r:tt)*) => {
//         let mut s = {
//             use std::io::Read;
//             let mut s = String::new();
//             std::io::stdin().read_to_string(&mut s).unwrap();
//             s
//         };
//         let mut iter = s.split_whitespace();
//         input_inner!{iter, $($r)*}
//     };
// }

// macro_rules! input_inner {
//     ($iter:expr) => {};
//     ($iter:expr, ) => {};

//     ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
//         let $var = read_value!($iter, $t);
//         input_inner!{$iter $($r)*}
//     };
// }

// macro_rules! read_value {
//     ($iter:expr, ( $($t:tt),* )) => {
//         ( $(read_value!($iter, $t)),* )
//     };

//     ($iter:expr, [ $t:tt ; $len:expr ]) => {
//         (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
//     };

//     ($iter:expr, chars) => {
//         read_value!($iter, String).chars().collect::<Vec<char>>()
//     };

//     ($iter:expr, usize1) => {
//         read_value!($iter, usize) - 1
//     };

//     ($iter:expr, $t:ty) => {
//         $iter.next().unwrap().parse::<$t>().expect("Parse error")
//     };
// }
