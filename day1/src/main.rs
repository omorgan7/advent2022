use std::vec::Vec;

fn main() {
    let input = include_str!("../input.txt");
    let mut calories = Vec::<i64>::new();
    input.lines().fold(&mut calories, |best, line| {
        if best.is_empty() {
            best.push(0);
        }

        match line.parse::<i64>() {
            Ok(i) => *best.last_mut().unwrap() += i,
            Err(_) => best.push(0),
        }
        best
    });

    calories.sort_by(|a, b| a.cmp(b).reverse());
    println!("{}", calories.iter().take(3).sum::<i64>());
}
