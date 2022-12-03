use std::vec::Vec;

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        input.lines().fold(0, |s, line| {
            let line_bytes = line.as_bytes();
            let sz = line.len();
            let (left, right) = line_bytes.split_at(sz / 2);

            let common_item = left
                .iter()
                .find(|&&l| right.iter().find(|&&r| r == l) != None)
                .unwrap();

            let priority = if *common_item >= b'a' {
                (common_item - b'a' + 1) as i64
            } else {
                (common_item - b'A' + 27) as i64
            };

            s + priority
        })
    );

    println!(
        "{}",
        input
            .lines()
            .map(|l| l.as_bytes())
            .collect::<Vec<&[u8]>>()
            .chunks(3)
            .fold(0, |s, lines| {
                let common_item = lines
                    .iter()
                    .next()
                    .unwrap()
                    .iter()
                    .filter_map(|a| lines.get(1).unwrap().iter().find(|&&b| *a == b))
                    .filter_map(|a| lines.get(2).unwrap().iter().find(|&&b| *a == b))
                    .next()
                    .unwrap();

                let priority = if *common_item >= b'a' {
                    (common_item - b'a' + 1) as i64
                } else {
                    (common_item - b'A' + 27) as i64
                };

                s + priority
            })
    );
}
