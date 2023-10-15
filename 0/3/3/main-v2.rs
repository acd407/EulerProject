fn gcd(x:usize,y:usize) -> usize {
    let (mut b, mut l) = (x, y);
    if b < l {
        (b,l) = (l,b)
    }
    while l != 0 {
        b %= l;
        (b,l) = (l,b)
    }
    b
}
fn bits(x: usize) -> usize {
    let mut x = x;
    let mut ans = 0;
    while x != 0 && x % 10 != 0 {
        ans ^= 1 << (x % 10 - 1);
        x /= 10;
    }
    ans
}

fn main() {
    let mut v = vec![];
    for i in 10..99 {
        for j in (i + 1)..=99 {
            let t = bits(i)&bits(j);
            if t.count_ones() != 1 {continue;}
            let ri = (bits(i)^t).trailing_zeros() as usize + 1;
            let rj = (bits(j)^t).trailing_zeros() as usize + 1;
            if ri * j == rj * i {
                v.push((ri, rj));
            }
        }
    }
    let i = v.iter().map(|x| x.0).product::<usize>();
    let j = v.iter().map(|x| x.1).product::<usize>();
    println!("{}", j / gcd(i, j));
}
