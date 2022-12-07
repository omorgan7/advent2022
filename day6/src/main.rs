use itertools::Itertools;

fn part1(input: &[u8], window_size: usize) -> i64 {
    for (idx, chunk) in input.windows(window_size).enumerate() {
        if chunk.iter().unique().count() == window_size {
            return (idx + window_size) as i64;
        }
    }
    panic!();
}

fn main() {
    let input = include_str!("../input.txt").as_bytes();
    println!("{}", part1(&input, 4));
    println!("{}", part1(&input, 14));
}
