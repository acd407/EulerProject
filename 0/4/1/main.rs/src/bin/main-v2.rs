use primal::*;

struct Permutation {
    current: Vec<u32>,
    stack: Vec<usize>,
}

impl Permutation {
    fn new(n: u32) -> Permutation {
        Permutation {
            current: (1..=n).collect(),
            stack: vec![0; n as usize],
        }
    }
}
impl Iterator for Permutation {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.current.len();
        let mut i = 0;
        while i < n {
            self.stack[i] += 1;
            if self.stack[i] > i {
                if i % 2 == 0 {
                    self.current.swap(0, i);
                } else {
                    self.current.swap(self.stack[i], i);
                }
                self.stack[i] = 0;
                i += 1;
            } else {
                self.current.swap(i - self.stack[i] + 1, i);
                return Some(self.current.clone());
            }
        }
        None
    }
}

fn from_vec(v: Vec<u32>) -> u32 {
    let mut ret = 0;
    for i in v {
        ret *= 10;
        ret += i;
    }
    ret
}

fn main() {
    let iter = Permutation::new(7);
    if let Some(ans) = iter.map(|v| from_vec(v)).filter(|&x| is_prime(x as u64)).max() {
        println!("{ans}");
    }
}
