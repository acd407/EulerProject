fn main() {
    for a in 1..=333 {
        for b in a..=(500 - a / 2) {
            if 1000 * (a + b) == a * b + 500000 {
                println!("{}^2 + {}^2 = {}^2", a, b, 1000 - a - b);
            }
        }
    }
}
