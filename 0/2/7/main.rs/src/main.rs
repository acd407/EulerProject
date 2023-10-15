extern crate primal;
use primal::*;

fn f(a: i64, b: i64) -> usize {
    for n in 0.. {
        if (a + n) * n + b <= 1 || ((a + n) * n + b > 1 && !is_prime(((a + n) * n + b) as u64)) {
            return n as usize;
        }
    }
    0
}

fn main() {
    let mut max_i = 0;
    let mut max_j = 0;
    let mut max_n = 0;
    for i in -999..999 {
        for j in -999..999 {
            let n = f(i as i64, j as i64);
            if n > max_n {
                max_i = i;
                max_j = j;
                max_n = n;
            }
        }
    }
    println!("b: {}\nc: {}\nn: {}\nans: {}", max_i, max_j, max_n, max_i * max_j);
}
