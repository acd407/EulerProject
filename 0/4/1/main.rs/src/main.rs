use primal::*;

fn pandigital_numbers(n: usize) -> impl Iterator<Item = u64> {
    let start = 10u64.pow(n as u32 - 1);
    let end = 10u64.pow(n as u32) - 1;
    
    (start..=end).filter(move |&num| {
        let mut digits = [false; 10];
        let mut temp = num;
        
        while temp > 0 {
            let digit = temp % 10;
            if digit == 0 || digits[digit as usize] {
                return false;
            }
            
            digits[digit as usize] = true;
            temp /= 10;
        }
        
        for i in 1..=n {
            if ! digits[i] {
                return false;
            }
        }
        true
    })
}

fn main() {
    if let Some(ans) = pandigital_numbers(7).filter(|&x| is_prime(x as u64)).max() {
        println!("{ans}");
    }
}
