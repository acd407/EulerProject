use std::io::Read;

fn f(i: i32, s: &str) -> i32 {
    let mut count = 0;
    for c in s.as_bytes() {
        count += (*c as i32) - ('A' as i32) + 1;
    }
    count * i
}

fn main() {
    let mut file = std::fs::File::open("names.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let len = contents.len();
    let contents = &contents[1..len - 2];
    let mut names: Vec<&str> = contents.split("\",\"").collect();
    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!(
        "{}",
        names.iter()
            .enumerate()
            .map(|(index, item)| f(index as i32 + 1, item) as u64)
            .sum::<u64>()
    );
}
