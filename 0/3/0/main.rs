fn main() {
    let mut v = Vec::new();
    for o in 1..=(1e6 - 1.) as u128 {
        let bits = &mut [0; 6];
        let mut i = o;
        let mut j = 0;
        while i != 0 {
            bits[j] = i % 10;
            j += 1;
            i /= 10;
        }
        if bits
            .iter()
            .map(|x| {
                let t = x * x;
                t * t * x
            })
            .sum::<u128>()
            == o
        {
            v.push(o);
        }
    }
    println!("{}", v.iter().sum::<u128>() - 1);
}
