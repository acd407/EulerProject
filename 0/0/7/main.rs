struct PT {
    prime_table: Vec<u64>,
    table_size: usize,
    i: usize,
}

impl PT {
    fn new(t: usize) -> PT {
        PT {
            prime_table: [2].to_vec(),
            table_size: t,
            i: 0,
        }
        .mktab()
    }
    fn mktab(mut self) -> Self {
        let mut i: u64 = *self.prime_table.last().unwrap();
        i = i / 2 * 2 + 1;
        while self.prime_table.len() < self.table_size {
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
        if self.i == self.prime_table.len() {
            None
        } else {
            self.i += 1;
            Some(self.prime_table[self.i - 1])
        }
    }
}

fn main() {
    let fb: PT = PT::new(10001);
    println!("{}", fb.last().unwrap());
}
