fn lexicographic_permutation(mut chars: Vec<u8>, mut i: u64) -> Vec<u8> {
    let l = chars.len() as u64;
    assert!(i < (1..=l).product::<u64>());
    let c = chars.remove((i / (1..l).product::<u64>()) as usize);
    i %= (1..l).product::<u64>();
    if l == 1 {
        assert!(i == 0);
        return vec![c];
    }
    let mut v = lexicographic_permutation(chars, i);
    v.insert(0, c);
    v
}

fn main() {
    println!(
        "{}",
        String::from_utf8(lexicographic_permutation(
            "0123456789".as_bytes().to_vec(),
            1e6 as u64 - 1
        ))
        .unwrap()
    )
}
