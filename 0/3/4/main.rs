fn bits_factorial(x: u128) -> u128 {
    let mut x = x;
    let mut sum = 0;
    while x != 0 {
        sum += (1..=(x%10)).product::<u128>();
        x /= 10;
    }
    sum
}

fn main () {
    let mut sum = 0;
    for i in 1..99999999 {
        if i == bits_factorial(i) {
            sum += i;
        }
    }
    println!("{}", sum - 3);
}
