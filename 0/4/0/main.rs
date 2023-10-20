// count bits before 10^l
fn cb(l: u32) -> u32 {
    if l == 0 {
        return 0;
    }
    9 * 10u32.pow(l - 1) * l + cb(l - 1)
}

// 返回第i位所在的数字以及偏移量
fn b2n(x: u32) -> (u32, u32) {
    let mut base_l = 1;
    while cb(base_l) < x {
        base_l += 1;
    }
    base_l -= 1;
    (
        (x - cb(base_l)) / (base_l + 1) + 10u32.pow(base_l) - 1,
        (x - cb(base_l)) % (base_l + 1),
    )
}

fn d(x: u32) -> u32 {
    let (b, mut i) = b2n(x);
    if i == 0 {
        return b % 10;
    }
    let mut y = b + 1;
    i = y.ilog10() + 2 - i;
    let mut r = 0;
    while i > 0 {
        r = y % 10;
        y /= 10;
        i -= 1;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cb() {
        assert_eq!(cb(1), 9);
        assert_eq!(cb(2), 189);
    }
    #[test]
    fn test_b2n() {
        assert_eq!(b2n(8), (8, 0));
        assert_eq!(b2n(9), (9, 0));
        assert_eq!(b2n(100), (54, 1));
        assert_eq!(b2n(189), (99, 0));
        assert_eq!(b2n(191), (99, 2));
        assert_eq!(b2n(192), (100, 0));
    }
    #[test]
    fn test_d() {
        assert_eq!(d(1), 1);
        assert_eq!(d(12), 1);
        assert_eq!(d(13), 1);
        assert_eq!(d(100), 5);
    }
}

fn main() {
    println!("{}", (0..=6).map(|x| d(10u32.pow(x))).product::<u32>())
}
