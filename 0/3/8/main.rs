fn bits(x: u32) -> u32 {
    let mut x = x;
    let mut ans = 0;
    while x != 0 {
        if x%10 == 0 {
            return 0;
        }
        let y = 1 << (x % 10 - 1);
        if ans & y != 0 {
            return 0;
        }
        ans |= y;
        x /= 10;
    }
    ans
}

fn is_pandigital(x: u32) -> bool {
    let mut tot = 0;
    for i in 1..=9 {
        let b = bits(i*x);
        if b == 0 || tot & b != 0 {
            return false;
        }
        tot |= b;
        if tot == 0b111111111 {
            return true;
        }
    }
    false
}

fn pandigital(x: u32) -> u32 {
    let v = &mut [0;9];
    let mut vi = 0;
    let mut i = 1;
    while vi != 9 {
        let mut y = x*i;
        while y != 0 {
            v[vi] = y / 10u32.pow(y.ilog10());
            vi += 1;
            y %= 10u32.pow(y.ilog10());
        }
        i += 1;
    }
    let mut sum = 0;
    for i in 0..=8 {
        sum += v[8-i] * 10u32.pow(i as u32);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_pandigital() {
        assert!(is_pandigital(192));
        assert!(is_pandigital(9));
        assert!(is_pandigital(1));
        assert!(!is_pandigital(191));
    }
    #[test]
    fn test_pandigital() {
        assert_eq!(pandigital(9),918273645);
        assert_eq!(pandigital(192),192384576);
    }
}

fn main () {
    // 要使 n > 1, 最多是4位数
    let v:Vec<u32> = (1..1e5 as u32).filter(|&x| is_pandigital(x)).collect();
    let p:Vec<u32> = v.into_iter().map(|x| pandigital(x)).collect();
    println!("{}", p.iter().max().unwrap());
}
