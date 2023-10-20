fn f(x: usize) -> usize {
    let mut count = 0;
    for i in 1..x/3 {
        for j in i..x/2 {
            if i*i + j*j == (x-i-j)*(x-i-j) {
                count += 1;
            }
        }
    }
    count
}

fn main () {
    let mut max=0;
    let mut max_i = 0;
    for i in 1..1000 {
        if f(i) > max {
            max = f(i);
            max_i = i;
        }
    }
    println!("{max} {max_i}");
}
