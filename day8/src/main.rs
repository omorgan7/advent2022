use std::fmt::Formatter;
use std::vec::Vec;

struct Grid {
    trees: Vec<i64>,
    width: usize,
    height: usize,
}

impl Grid {
    fn at(&self, x: i64, y: i64) -> i64 {
        self.trees[(self.width as i64 * y + x) as usize]
    }

    fn top(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut nbrs = Vec::<(i64, i64)>::new();

        for yy in 0..y as i64 {
            nbrs.push((x, yy));
        }

        nbrs.reverse();
        nbrs
    }

    fn bottom(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut nbrs = Vec::<(i64, i64)>::new();

        for yy in y + 1..self.height as i64 {
            nbrs.push((x, yy));
        }

        nbrs
    }

    fn left(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut nbrs = Vec::<(i64, i64)>::new();

        for xx in 0..x as i64 {
            nbrs.push((xx, y));
        }

        nbrs.reverse();
        nbrs
    }

    fn right(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        let mut nbrs = Vec::<(i64, i64)>::new();

        for xx in x + 1..self.width as i64 {
            nbrs.push((xx, y));
        }

        nbrs
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.trees[self.width * y + x])?
            }
            if y != self.height - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

fn part1(grid: &Grid) -> i64 {
    let mut count = 0;
    for y in 1..(grid.height - 1) as i64 {
        for x in 1..(grid.width - 1) as i64 {
            let neighbours = [
                grid.top(x, y),
                grid.bottom(x, y),
                grid.left(x, y),
                grid.right(x, y),
            ];

            if neighbours
                .iter()
                .any(|nb| nb.iter().all(|(nx, ny)| grid.at(*nx, *ny) < grid.at(x, y)))
            {
                count += 1;
            }
        }
    }
    count + grid.width as i64 * 2 + (grid.height as i64 - 2) * 2
}

fn part2(grid: &Grid) -> i64 {
    let mut best = 0;
    for y in 1..(grid.height - 1) as i64 {
        for x in 1..(grid.width - 1) as i64 {
            let neighbours = [
                grid.top(x, y),
                grid.bottom(x, y),
                grid.left(x, y),
                grid.right(x, y),
            ];

            best = std::cmp::max(
                best,
                neighbours
                    .iter()
                    .map(|nb| {
                        nb.iter()
                            .enumerate()
                            .find(|(_, (nx, ny))| grid.at(*nx, *ny) >= grid.at(x, y))
                            .unwrap_or((nb.len() - 1, &(0, 0)))
                            .0 as i64
                            + 1
                    })
                    .product(),
            );
        }
    }
    best
}

fn main() {
    let input = include_str!("../input.txt");

    let width = input.lines().next().unwrap().len();
    let trees = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<i64>>();
    let height = trees.len() / width;
    let grid = Grid {
        trees,
        width,
        height,
    };
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}
