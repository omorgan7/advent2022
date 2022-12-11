use itertools::Itertools;
use std::cmp::Reverse;
use std::fmt::Formatter;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum Op {
    Add(i64),
    Mul(i64),
    SelfMul,
}

#[derive(Debug, Clone)]
enum Op2 {
    Num(i64),
    Add(i64),
    Mul(i64),
    SelfMul,
}

impl Op {
    fn check(&self, x: i64) -> i64 {
        match self {
            Op::Add(y) => x + y,
            Op::Mul(y) => x * y,
            Op::SelfMul => x * x,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_by: i64,
    throw_if_true: i64,
    throw_if_false: i64,
    inspections: i64,
}

struct Monkeys(Vec<Monkey>);

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Inspections: {} ", self.inspections)?;
        write!(f, "Op: {:?} ", self.op)?;
        write!(f, "div: {} ", self.divisible_by)?;
        write!(f, "throw_true: {} ", self.throw_if_true)?;
        write!(f, "throw_false: {} ", self.throw_if_false)?;
        write!(f, "Items: {:?}", self.items)?;

        Ok(())
    }
}

impl std::fmt::Display for Monkeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (idx, m) in self.0.iter().enumerate() {
            write!(f, "Monkey {} {{{}}}", idx, m)?;
            if idx != self.0.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn part1(m: &[Monkey]) -> i64 {
    let mut monkeys = m.to_vec();
    let mut items = monkeys
        .iter()
        .map(|m| m.items.clone())
        .collect::<Vec<Vec<i64>>>();

    for _ in 0..20 {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            monkey.items = items[idx].clone();

            for item in &monkey.items {
                let worry = monkey.op.check(*item) / 3;
                if worry % monkey.divisible_by == 0 {
                    items[monkey.throw_if_true as usize].push(worry);
                } else {
                    items[monkey.throw_if_false as usize].push(worry);
                }
            }

            monkey.inspections += monkey.items.len() as i64;

            monkey.items.clear();
            items[idx].clear();
        }
    }

    for (idx, monkey) in monkeys.iter_mut().enumerate() {
        monkey.items = items[idx].clone();
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .map(Reverse)
        .k_smallest(2)
        .map(|x| x.0)
        .product::<i64>()
}

fn part2(m: &[Monkey]) -> i64 {
    let mut monkeys = m.to_vec();

    let mut start_ops = monkeys
        .iter()
        .map(|monkey| {
            monkey
                .items
                .iter()
                .map(|i| [Op2::Num(*i); 1].to_vec())
                .collect::<Vec<Vec<Op2>>>()
        })
        .collect::<Vec<Vec<Vec<Op2>>>>();
    let mut ops = start_ops.clone();

    for _ in 0..10000 {
        for (idx, monkey) in monkeys.iter_mut().enumerate() {
            let div = monkey.divisible_by;

            ops[idx].iter_mut().for_each(|o| {
                o.push(match monkey.op {
                    Op::Add(x) => Op2::Add(x),
                    Op::Mul(x) => Op2::Mul(x),
                    Op::SelfMul => Op2::SelfMul,
                })
            });

            start_ops[idx] = ops[idx].clone();

            for item in &start_ops[idx] {
                let mut worry = match item[0] {
                    Op2::Num(x) => x,
                    _ => panic!(),
                };

                for op in &item[1..] {
                    match op {
                        Op2::Add(x) => worry = (worry % div) + (x % div),
                        Op2::Mul(x) => worry = (worry % div) * (x % div),
                        Op2::SelfMul => worry = (worry % div) * (worry % div),
                        _ => {}
                    }
                }

                if worry % div == 0 {
                    ops[monkey.throw_if_true as usize].push(item.clone());
                } else {
                    ops[monkey.throw_if_false as usize].push(item.clone());
                }
            }

            monkey.inspections += (start_ops[idx].len()) as i64;
            start_ops[idx].clear();
            ops[idx].clear();
        }
    }

    monkeys
        .iter()
        .map(|m| m.inspections)
        .map(Reverse)
        .k_smallest(2)
        .map(|x| x.0)
        .product::<i64>()
}

fn main() {
    let input = include_str!("../input.txt");

    let monkeys = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|lines| {
            let mut line = lines.iter().skip(1);
            let starting_items = line.next().unwrap()[18..]
                .split(' ')
                .map(|s| s.trim_matches(',').parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut ops = line.next().unwrap()[23..].split(' ');
            let op = match ops.next().unwrap().chars().next().unwrap() {
                '*' => {
                    let mult = ops.next().unwrap();
                    if mult == "old" {
                        Op::SelfMul
                    } else {
                        Op::Mul(mult.parse::<i64>().unwrap())
                    }
                }
                _ => Op::Add(ops.next().unwrap().parse::<i64>().unwrap()),
            };

            let divisible_by = line.next().unwrap()[21..].parse::<i64>().unwrap();
            let throw_if_true = line.next().unwrap()[29..].parse::<i64>().unwrap();
            let throw_if_false = line.next().unwrap()[30..].parse::<i64>().unwrap();

            Monkey {
                items: starting_items,
                op,
                divisible_by,
                throw_if_true,
                throw_if_false,
                inspections: 0,
            }
        })
        .collect::<Vec<Monkey>>();

    println!("{}", part1(&monkeys));
    println!("{}", part2(&monkeys));
}
