// n >= 2
fn d(n:u64) -> u64 {
    let ns = (n as f32).sqrt() as u64;
    let mut v = vec![];
    for i in 2..ns {
        if n%i == 0 {
            v.push(i);
            v.push(n/i);
        }
    } 
    if n == ns * ns {
        v.push(ns);
    }
    v.iter().sum::<u64>() + 1
}

fn main () {
    let mut u = vec![];
    // d(1) = ()
    // d(2) = 1, d(d(2)) = ()
    for i in 3..10000 {
        if d(d(i)) == i && d(i) != i {
            u.push(i);
        }
    }
    println!("{}",u.iter().sum::<u64>());
}
