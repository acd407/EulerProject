struct PT {
    prime_table: Vec<u64>,
    target: u64,
    p: usize,
}

impl PT {
    fn new(t: u64) -> PT {
        PT {
            prime_table: [2].to_vec(),
            target: t,
            p: 0,
        }
    }
    fn mktab(mut self) -> Self {
        let mut i: u64 = *self.prime_table.last().unwrap();
        i = i / 2 * 2 + 1;
        while i < self.target {
            let mut for_end = true;
            for j in &self.prime_table {
                if i % j == 0 {
                    for_end = false;
                    break;
                }
            }
            if for_end {
                self.prime_table.push(i);
            }
            i += 2;
        }
        self
    }
}

impl Iterator for PT {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.p == self.prime_table.len() {
            None
        } else {
            self.p += 1;
            Some(self.prime_table[self.p - 1])
        }
    }
}

fn main() {
    let fb: PT = PT::new(600851475143_f64.sqrt() as u64).mktab();
    // let fb:PT = PT::new(8000).mktab();
    // let _: Vec<_> = fb.map(|x| println!("{}", x)).collect();
    println!("{}", fb.last().unwrap());
}
