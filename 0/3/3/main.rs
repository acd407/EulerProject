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

fn main() {
    let mut v = vec![];
    for i in 10..99 {
        for j in (i + 1)..=99 {
            // i/j
            let (mut ih, mut il) = (i / 10, i % 10);
            let (mut jh, mut jl) = (j / 10, j % 10);
            if i%10==0||j%10==0 {continue;}
            if ih == jh {
                ih = 0;
                jh = 0;
            } else if il == jl {
                il = 0;
                jl = 0;
            } else if ih == jl {
                ih = 0;
                jl = 0;
            } else if il == jh {
                il = 0;
                jh = 0;
            } else {
                continue;
            }
            let ri = ih + il;
            let rj = jh + jl;
            if ri * j == rj * i {
                v.push((ri, rj));
            }
        }
    }
    let i = v.iter().map(|x| x.0).product::<usize>();
    let j = v.iter().map(|x| x.1).product::<usize>();
    println!("{}", j / gcd(i, j));
}
