use prime_factorization::Factorization;
use std::collections::HashSet;
fn main() {
    let mut table = HashSet::new();
    for a in 2..=100 as u32 {
        for b in 2..=100 as u32 {
            let factor_repr = Factorization::<u32>::run(a);
            let factors = factor_repr.prime_factor_repr();
            let v = factors
                .iter()
                .map(|&(f, i)| (f, i * b))
                .collect::<Vec<(u32, u32)>>();
            table.insert(v);
        }
    }
    println!("{}", table.len());
}
