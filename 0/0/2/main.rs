struct Fib {
    last: i32,
    cur: i32,
}

impl Fib {
    fn new() -> Fib {
        Fib { last: 1, cur: 1 }
    }
}

impl Iterator for Fib {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let ans = self.last + self.cur;
        if ans >= 4e6 as i32 {
            return None;
        }
        self.last = self.cur;
        self.cur = ans;
        Some(ans)
    }
}

fn main() {
    let fb = Fib::new();
    println!("{}", fb.filter(|x| x%2 == 0).sum::<i32>());
}
