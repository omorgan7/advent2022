use either::*;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::vec::Vec;

type ListOrInt = either::Either<List, i64>;

#[derive(Debug, Clone)]
struct List {
    list: Vec<ListOrInt>,
}

fn recursive_parse<'a>(l: &'a [char]) -> (List, &'a [char]) {
    let mut line = l;
    let mut list = List { list: vec![] };

    let mut number_to_parse = String::new();

    loop {
        match line[0] {
            ',' => {
                if let Ok(x) = number_to_parse.parse::<i64>() {
                    list.list.push(Right(x));
                }
                number_to_parse.clear();
                line = &line[1..];
            }
            '[' => {
                let next = recursive_parse(&line[1..]);
                list.list.push(Left(next.0));
                line = next.1;
            }
            ']' => {
                if let Ok(x) = number_to_parse.parse::<i64>() {
                    list.list.push(Right(x));
                }

                return (list, &line[1..]);
            }

            x => {
                number_to_parse.push(x);
                line = &line[1..];
            }
        }
    }
}

fn recursive_check<'a, 'b>(l: (&'a [ListOrInt], &'b [ListOrInt])) -> std::cmp::Ordering {
    let mut first = l.0;
    let mut second = l.1;
    loop {
        if first.is_empty() && !second.is_empty() {
            return Less;
        }

        if first.is_empty() && second.is_empty() {
            return Equal;
        }

        if !first.is_empty() && second.is_empty() {
            return Greater;
        }

        match &first[0] {
            Right(x) => {
                match &second[0] {
                    Right(y) => {
                        if x > y {
                            return Greater;
                        } else if x < y {
                            return Less;
                        }
                    }
                    Left(list) => {
                        // Promote to list, and recurse.
                        let promotion = List {
                            list: vec![Right(*x)],
                        };

                        let result = recursive_check((&promotion.list[0..], &list.list[0..]));
                        if result != Equal {
                            return result;
                        }
                    }
                }
            }
            Left(left_list) => {
                match &second[0] {
                    Right(y) => {
                        // Promote to list, and recurse.
                        let promotion = List {
                            list: vec![Right(*y)],
                        };

                        let result = recursive_check((&left_list.list[0..], &promotion.list[0..]));
                        if result != Equal {
                            return result;
                        }
                    }
                    Left(right_list) => {
                        let result = recursive_check((&left_list.list[0..], &right_list.list[0..]));
                        if result != Equal {
                            return result;
                        }
                    }
                }
            }
        }

        first = &first[1..];
        second = &second[1..];
    }
}

fn part1(list: &[(List, List)]) -> i64 {
    list.iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            let first = &pair.0.list[0..];
            let second = &pair.1.list[0..];

            let sorted = recursive_check((first, second));
            match sorted {
                Less => Some(idx as i64 + 1),
                _ => None,
            }
        })
        .sum::<i64>()
}

fn part2(list: &[List]) -> i64 {
    let mut sorted = list.to_vec();

    sorted.sort_by(|a, b| recursive_check((&a.list, &b.list)));

    let find = |i: i64| -> i64 {
        for (idx, l) in sorted.iter().enumerate() {
            if l.list.is_empty() {
                continue;
            }
            if let Left(list) = &l.list[0] {
                if list.list.is_empty() {
                    continue;
                }
                if let Right(x) = list.list[0] {
                    if x == i && list.list.len() == 1 && l.list.len() == 1 {
                        return idx as i64 + 1;
                    }
                }
            }
        }
        panic!();
    };

    find(2) * find(6)
}

fn main() {
    let input = include_str!("../input.txt");

    let lists = input
        .split("\n\n")
        .map(|twolines| {
            let mut iter = twolines.lines().map(|line| {
                let chars = line.chars().collect::<Vec<char>>();
                let result = recursive_parse(&chars[1..]);
                result.0
            });

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect::<Vec<(List, List)>>();

    let lists_flat = {
        let mut tmp = input
            .split("\n\n")
            .flat_map(|twolines| {
                let mut iter = twolines.lines().map(|line| {
                    let chars = line.chars().collect::<Vec<char>>();
                    let result = recursive_parse(&chars[1..]);
                    result.0
                });

                [iter.next().unwrap(), iter.next().unwrap()]
            })
            .collect::<Vec<List>>();
        let two = List {
            list: vec![Left(List {
                list: vec![Right(2)],
            })],
        };
        let six = List {
            list: vec![Left(List {
                list: vec![Right(6)],
            })],
        };
        tmp.push(two);
        tmp.push(six);
        tmp
    };

    println!("{}", part1(&lists));
    println!("{}", part2(&lists_flat));
}
