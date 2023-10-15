use primal::*;
use std::collections::HashSet;

fn prime_l(x: u64) -> bool {
    if x == 0 {
        return true;
    }
    if !is_prime(x) {
        return false;
    }
    let n_digits = x.ilog10();
    prime_l(x - x / 10u64.pow(n_digits) * 10u64.pow(n_digits))
}

fn prime_r(x: u64) -> bool {
    if x == 0 {
        return true;
    }
    if !is_prime(x) {
        return false;
    }
    prime_r(x / 10)
}

// another sol
fn sol() {
    let mut hs = HashSet::new();
    for i in 1..1e6 as u64 {
        if prime_r(i) && prime_l(i) {
            println!("{i}");
            hs.insert(i);
        }
    }
    println!("{}", hs.into_iter().sum::<u64>());
}

fn main() {
    let mut v = vec![2, 3, 5, 7];
    let mut bit = 1;
    while !v.is_empty() {
        let mut r = Vec::new();
        for i in v {
            for j in 1..10 {
                let x = i + j * 10u64.pow(bit);
                if is_prime(x) {
                    r.push(x);
                }
            }
        }
        v = r;
        bit += 1;
        for i in &v {
            if *i > 1e10 as u64 {
                return ();
            }
            if prime_r(*i) {
                println!("{i}");
            }
        }
    }
}
