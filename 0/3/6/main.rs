fn palindrome(x: u32) -> bool {
    let mut x = x;
    let bits: &mut [u8] = &mut [0; 10];
    let mut i: usize = 0;
    while x != 0 {
        bits[i] = (x % 10) as u8;
        i += 1;
        x /= 10;
    }
    for j in 0..i / 2 {
        if bits[j] != bits[i - 1 - j] {
            return false;
        }
    }
    true
}

fn palindrome2(x: u32) -> bool {
    let len = u32::BITS - x.leading_zeros();
    for j in 0..len / 2 {
        if ((x >> j) ^ (x >> (len - 1 - j))) & 1 != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_palindrome() {
        assert!(palindrome(123321));
        assert!(palindrome(12321));
        assert!(!palindrome(2321));
        assert!(palindrome(1));
        assert!(palindrome(585));
    }
    #[test]
    fn test_palindrome2() {
        assert!(palindrome2(0b101101));
        assert!(palindrome2(0b101));
        assert!(!palindrome2(0b1010));
        assert!(palindrome2(0b1));
        assert!(palindrome2(585));
    }
}

fn main() {
    let mut sum = 0;
    for i in 1..1e6 as u32 {
        if palindrome2(i) && palindrome(i) {
            sum += i;
        }
    }
    println!("{sum}");
}
