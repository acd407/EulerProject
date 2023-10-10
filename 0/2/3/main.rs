fn factors(x: i32) -> Vec<i32> {
    let mut pt: Vec<i32> = vec![1];
    let mut x_sq = (x as f32).sqrt() as i32;
    if x == x_sq * x_sq && x > 1 {
        pt.push(x_sq);
        x_sq -= 1;
    }
    for i in 2..=x_sq {
        if x % i == 0 {
            pt.push(i);
            pt.push(x / i);
        }
    }
    pt
}

fn main() {
    let mut abundant = Vec::new();
    for i in 1..=28123 {
        if factors(i).iter().sum::<i32>() > i {
            abundant.push(i);
        }
    }
    let mut number_table: Vec<bool> = vec![true; 28124];
    for i in &abundant {
        for j in &abundant {
            if i + j < 28124 {
                number_table[(i + j) as usize] = false;
            }
        }
    }
    println!(
        "{}",
        number_table
            .iter()
            .enumerate()
            .map(|(i, &x)| if x { i as i32 } else { 0 })
            .sum::<i32>()
    )
}
