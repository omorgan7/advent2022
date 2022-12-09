use std::collections::HashSet;
use std::fmt::Formatter;
use std::vec::Vec;

extern crate termion;
use termion::clear::All;

use std::{thread, time};

#[derive(Clone)]
struct Grid {
    positions: HashSet<(i64, i64)>,
    head: (i64, i64),
    tail: (i64, i64),
    tails: [(i64, i64); 10],
}

impl Grid {
    fn distance(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
        ((head.0 - tail.0).abs(), (head.1 - tail.1).abs())
    }

    fn translate_pt1(&mut self, dx: i64, dy: i64) {
        self.positions.insert(self.tail);

        self.head.0 += dx;
        self.head.1 += dy;

        let delta = Grid::distance(self.head, self.tail);

        match delta {
            (2, 0) => {
                self.tail.0 += (self.head.0 - self.tail.0).signum();
            }
            (0, 2) => {
                self.tail.1 += (self.head.1 - self.tail.1).signum();
            }
            (2, 1) | (1, 2) | (2, 2) => {
                self.tail.0 += (self.head.0 - self.tail.0).signum();
                self.tail.1 += (self.head.1 - self.tail.1).signum();
            }
            _ => {}
        }
    }

    fn translate_pt2(&mut self, dx: i64, dy: i64) {
        self.positions.insert(self.tails[9]);

        self.tails[0].0 += dx;
        self.tails[0].1 += dy;

        for i in 1..10 {
            let (s0, s1) = self.tails.split_at_mut(i);
            let head = s0.last_mut().unwrap();
            let tail = s1.first_mut().unwrap();

            let delta = Grid::distance(*head, *tail);

            match delta {
                (2, 0) => {
                    tail.0 += (head.0 - tail.0).signum();
                }
                (0, 2) => {
                    tail.1 += (head.1 - tail.1).signum();
                }
                (2, 1) | (1, 2) | (2, 2) => {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                }
                _ => {}
            }
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let width = 20;
        let height = 20;
        for y in 0..height {
            for x in 0..width {
                let p = (
                    x as i64 - width / 2 + self.tails[0].0,
                    y as i64 - height / 2 + self.tails[0].1,
                );

                let idx = self
                    .tails
                    .iter()
                    .enumerate()
                    .find(|(_, t)| **t == p)
                    .unwrap_or((usize::MAX, &(0, 0)))
                    .0;

                if idx != usize::MAX {
                    if idx == 0 {
                        write!(f, "H")?
                    } else {
                        write!(f, "{}", idx)?
                    }
                } else {
                    write!(f, ".")?
                }
            }

            if y != height - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = Grid {
        positions: HashSet::new(),
        head: (0, 0),
        tail: (0, 0),
        tails: [(0, 0); 10],
    };

    let movements = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let direction = split.next().unwrap();
            let count = split.next().unwrap().parse::<i64>().unwrap();

            (direction.chars().next().unwrap(), count)
        })
        .collect::<Vec<(char, i64)>>();

    let mut grid_copy = grid.clone();

    for (direction, count) in movements.iter() {
        for _ in 0..*count {
            match direction {
                'U' => {
                    grid_copy.translate_pt1(0, -1);
                }
                'D' => {
                    grid_copy.translate_pt1(0, 1);
                }
                'L' => {
                    grid_copy.translate_pt1(-1, 0);
                }
                'R' => {
                    grid_copy.translate_pt1(1, 0);
                }
                _ => {}
            }
        }
    }

    // bump the tail setting
    grid_copy.translate_pt2(0, 0);
    println!("{}", grid_copy.positions.len());

    grid_copy = grid;

    for (direction, count) in movements.iter() {
        for _ in 0..*count {
            match direction {
                'U' => {
                    grid_copy.translate_pt2(0, -1);
                }
                'D' => {
                    grid_copy.translate_pt2(0, 1);
                }
                'L' => {
                    grid_copy.translate_pt2(-1, 0);
                }
                'R' => {
                    grid_copy.translate_pt2(1, 0);
                }
                _ => {}
            }
        }

        // thread::sleep(time::Duration::from_millis(10));
        // print!("{}", termion::clear::All);
        // println!("{}\n", grid_copy);
    }

    grid_copy.translate_pt2(0, 0);
    println!("{}", grid_copy.positions.len());
}
