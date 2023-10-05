fn main() {
    let mut n = 600851475143_u64;
    let mut pt = Vec::new();

    while n != 1 {
        let mut for_end = true;
        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                pt.push(i);
                n /= i;
                for_end = false;
            }
        }
        if for_end {
            pt.push(n);
            n = 1;
        }
    }

    println!("{}", pt.iter().max().unwrap());
}
