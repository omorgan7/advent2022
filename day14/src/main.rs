use std::collections::HashMap;
use std::fmt::Formatter;
use std::vec::Vec;

#[derive(Clone, PartialEq)]
enum State {
    Empty,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Grid {
    min: (i64, i64),
    max: (i64, i64),
    grid: Vec<State>,
}

#[derive(Clone)]
struct Grid2 {
    min: (i64, i64),
    max: (i64, i64),
    grid: HashMap<(i64, i64), State>,
}

impl Grid {
    fn width(&self) -> i64 {
        self.max.0 - self.min.0 + 1
    }
    fn at(&mut self, x: i64, y: i64) -> &mut State {
        let width = self.width();
        &mut self.grid[(x - self.min.0 + (y - self.min.1) * width) as usize]
    }
    fn at_nonmut(&self, x: i64, y: i64) -> &State {
        let width = self.width();
        &self.grid[(x - self.min.0 + (y - self.min.1) * width) as usize]
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                match *self.at_nonmut(x, y) {
                    State::Empty => write!(f, ".")?,
                    State::Rock => write!(f, "#")?,
                    State::Sand => write!(f, "o")?,
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Grid2 {
    fn at(&mut self, x: i64, y: i64) -> &mut State {
        self.grid.entry((x, y)).or_insert(if y == self.max.1 {
            State::Rock
        } else {
            State::Empty
        })
        // &mut self.grid[(x - self.min.0 + (y - self.min.1) * width) as usize]
    }
    fn at_nonmut(&self, x: i64, y: i64) -> &State {
        self.grid.get(&(x, y)).unwrap_or(if y == self.max.1 {
            &State::Rock
        } else {
            &State::Empty
        })
    }
}

impl std::fmt::Display for Grid2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut min = (i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN);

        for s in self.grid.keys() {
            min.0 = std::cmp::min(min.0, s.0);
            max.0 = std::cmp::max(max.0, s.0);

            min.1 = std::cmp::min(min.1, s.1);
            max.1 = std::cmp::max(max.1, s.1);
        }

        min.0 -= 1;
        max.0 += 1;

        for y in min.1 - 1..=max.1 + 1 {
            for x in min.0 - 1..=max.0 + 1 {

                match self.at_nonmut(x, y) {
                    State::Empty => write!(f, ".")?,
                    State::Rock => write!(f, "#")?,
                    State::Sand => write!(f, "o")?,
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn part2(mut grid: Grid2) -> i64 {
    let spawn_point = (500, 0);

    loop {
        let mut sand_position = spawn_point;
        let mut converged = false;
        loop {
            if *grid.at(sand_position.0, sand_position.1 + 1) == State::Empty {
                sand_position.1 += 1;
                continue;
            }

            if *grid.at(sand_position.0 - 1, sand_position.1 + 1) == State::Empty {
                sand_position.0 -= 1;
                sand_position.1 += 1;
                continue;
            }

            if *grid.at(sand_position.0 + 1, sand_position.1 + 1) == State::Empty {
                sand_position.0 += 1;
                sand_position.1 += 1;
                continue;
            }

            *grid.at(sand_position.0, sand_position.1) = State::Sand;
            if sand_position == spawn_point {
                converged = true;
            }

            break;
        }

        if converged {
            break;
        }
    }

    grid.grid
        .into_values()
        .fold(0i64, |acc, x| if x == State::Sand { acc + 1 } else { acc })
}

fn part1(mut grid: Grid) -> i64 {
    let spawn_point = (500, 0);

    let mut count = 0;
    loop {
        let mut sand_position = spawn_point;
        let mut converged = false;
        loop {
            if sand_position.1 >= grid.max.1 {
                converged = true;
                break;
            }

            if *grid.at(sand_position.0, sand_position.1 + 1) == State::Empty {
                sand_position.1 += 1;
                continue;
            }

            if *grid.at(sand_position.0 - 1, sand_position.1 + 1) == State::Empty {
                sand_position.0 -= 1;
                sand_position.1 += 1;
                continue;
            }

            if *grid.at(sand_position.0 + 1, sand_position.1 + 1) == State::Empty {
                sand_position.0 += 1;
                sand_position.1 += 1;
                continue;
            }

            *grid.at(sand_position.0, sand_position.1) = State::Sand;
            break;
        }

        if converged {
            break;
        }
        count += 1;
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");

    let walls = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let mut split = coord.split(',');
                    (
                        split.next().unwrap().parse::<i64>().unwrap(),
                        split.next().unwrap().parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<_>>();

    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);

    for w in &walls {
        for s in w {
            min.0 = std::cmp::min(min.0, s.0);
            max.0 = std::cmp::max(max.0, s.0);

            min.1 = std::cmp::min(min.1, s.1);
            max.1 = std::cmp::max(max.1, s.1);
        }
    }

    min.1 = std::cmp::min(min.1, 0);

    let width = max.0 - min.0;
    let height = max.1 - min.1;

    let mut grid = Grid {
        min,
        max,
        grid: vec![State::Empty; ((width + 1) * (height + 1)) as usize],
    };

    for y in grid.min.1..=grid.max.1 {
        for x in grid.min.0..=grid.max.0 {
            if walls.iter().any(|w| {
                w.windows(2).any(|wall| {
                    let xrange = {
                        if wall[0].0 > wall[1].0 {
                            wall[1].0..=wall[0].0
                        } else {
                            wall[0].0..=wall[1].0
                        }
                    };

                    let yrange = {
                        if wall[0].1 > wall[1].1 {
                            wall[1].1..=wall[0].1
                        } else {
                            wall[0].1..=wall[1].1
                        }
                    };
                    xrange.contains(&x) && yrange.contains(&y)
                })
            }) {
                *grid.at(x, y) = State::Rock;
            }
        }
    }

    println!("{}", part1(grid.clone()));

    max.1 += 2;
    let mut grid2 = Grid2 {
        min,
        max,
        grid: HashMap::new(),
    };

    for y in grid2.min.1..=grid2.max.1 - 2 {
        for x in grid2.min.0..=grid2.max.0 {
            if walls.iter().any(|w| {
                w.windows(2).any(|wall| {
                    let xrange = {
                        if wall[0].0 > wall[1].0 {
                            wall[1].0..=wall[0].0
                        } else {
                            wall[0].0..=wall[1].0
                        }
                    };

                    let yrange = {
                        if wall[0].1 > wall[1].1 {
                            wall[1].1..=wall[0].1
                        } else {
                            wall[0].1..=wall[1].1
                        }
                    };
                    xrange.contains(&x) && yrange.contains(&y)
                })
            }) {
                *grid2.at(x, y) = State::Rock;
            }
        }
    }

    for x in grid2.min.0..=grid2.max.0 {
        *grid2.at(x, grid2.max.1) = State::Rock;
    }

    println!("{}", part2(grid2.clone()));
}
