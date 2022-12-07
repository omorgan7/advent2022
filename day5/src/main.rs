use std::vec::Vec;

#[derive(Debug, Clone)]
struct Crate {
    label: char,
}

#[derive(Debug)]
struct Move {
    count: i64,
    from: i64,
    to: i64,
}

fn part1(moves: &Vec<Move>, c: &Vec<Vec<Crate>>) -> String {
    let mut crates = c.clone();

    for m in moves {
        for _ in 0..(m.count as usize) {
            let old = crates[m.from as usize - 1].pop().unwrap();
            crates[m.to as usize - 1].push(old);
        }
    }

    crates
        .iter()
        .map(|c| c.last().unwrap().label)
        .collect::<String>()
}

fn part2(moves: &Vec<Move>, c: &Vec<Vec<Crate>>) -> String {
    let mut crates = c.clone();

    for m in moves {
        let start_offset = crates[m.from as usize - 1].len() - m.count as usize;
        for i in 0..(m.count as usize) {
            let old = crates[m.from as usize - 1][start_offset + i].clone();
            crates[m.to as usize - 1].push(old);
        }
        crates[m.from as usize - 1].truncate(start_offset);
    }

    crates
        .iter()
        .map(|c| c.last().unwrap().label)
        .collect::<String>()
}

fn main() {
    let input = include_str!("../input.txt");

    let initial = input.split("\n\n").next().unwrap();

    let crate_count = (initial.lines().next().unwrap().len() + 1) / 4;

    let mut crates = Vec::<Vec<Crate>>::new();
    crates.resize(crate_count, vec![]);

    let line_count = initial.lines().count();

    for line in initial.lines().take(line_count - 1) {
        for i in 0..(crate_count as usize) {
            let label = line.as_bytes()[1 + i * 4] as char;
            if label != ' ' {
                crates[i].push(Crate { label: label });
            }
        }
    }

    for c in &mut crates {
        c.reverse();
    }

    let moves_str = input.split("\n\n").nth(1).unwrap();

    let moves = moves_str
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let count = split.nth(1).unwrap().parse::<i64>().unwrap();
            let from = split.nth(1).unwrap().parse::<i64>().unwrap();
            let to = split.nth(1).unwrap().parse::<i64>().unwrap();
            Move { count, from, to }
        })
        .collect::<Vec<Move>>();

    println!("{}", part1(&moves, &crates));
    println!("{}", part2(&moves, &crates));
}
