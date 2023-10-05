fn palindrome(x: i32) -> bool {
    let s = x.to_string();
    let s = s.as_bytes().iter();
    let s = s.clone().zip(s.rev());
    for i in s {
        if *i.0 != *i.1 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut pdt = Vec::new();
    for i in 100..1000 {
        for j in 100..1000 {
            if palindrome(i * j) {
                pdt.push(i * j);
            }
        }
    }

    println!("{}", pdt.iter().max().unwrap());
}
