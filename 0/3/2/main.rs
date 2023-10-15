use std::collections::HashSet;
fn bits(x: usize) -> u32 {
    let mut x = x;
    let mut ans = 0;
    while x != 0 && x % 10 != 0 {
        ans ^= 1 << (x % 10 - 1);
        x /= 10;
    }
    ans
}
fn main() {
    let mut v = HashSet::new();
    for i in 1..=9 {
        for j in 1..=9999 {
            if i * j > 9999 {
                break;
            }
            if bits(i) + bits(j) + bits(i * j) == 0b111111111 {
                v.insert(i * j);
                println!("{} * {} = {}", i, j, i * j)
            }
        }
    }
    for i in 1..=99 {
        for j in 1..=999 {
            if i * j > 9999 {
                break;
            }
            if bits(i) + bits(j) + bits(i * j) == 0b111111111 {
                v.insert(i * j);
                println!("{} * {} = {}", i, j, i * j)
            }
        }
    }
    println!("{}\n{}", v.len(), v.iter().sum::<usize>());
}
