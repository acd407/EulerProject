fn main() {
    let r1: i32 = (1..=100).map(|x| x * x).sum::<i32>();
    let r2: i32 = (1..=100).sum::<i32>();
    let r2: i32 = r2 * r2;
    println!("{}", r2 - r1);
}
