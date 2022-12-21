use std::collections::HashSet;

#[derive(PartialEq)]
enum Push {
    Left,
    Right,
}

struct Rock {
    positions: Vec<(i64, i64)>,
}

impl Rock {
    fn x_extent(&self) -> i64 {
        let to_x = |p: &(i64, i64)| p.0;
        let max = self.positions.iter().map(to_x).max().unwrap();
        let min = self.positions.iter().map(to_x).min().unwrap();

        max - min
    }
}

fn part1(pushes: &[Push], rocks: &[Rock], steps: usize) -> i64 {
    let mut push_iter = pushes.iter();
    let mut rock_iter = rocks.iter();

    let mut occupied = HashSet::<(i64, i64)>::new();

    let mut max_y = 0;

    for _ in 0..steps {
        let rock = match rock_iter.next() {
            Some(r) => r,
            None => {
                rock_iter = rocks.iter();
                rock_iter.next().unwrap()
            }
        };

        let mut position = (2, max_y + 4);
        loop {
            let intersection =
                |rock: &Rock, new_position: &(i64, i64), occupied: &HashSet<(i64, i64)>| -> bool {
                    rock.positions
                        .iter()
                        .any(|p| occupied.contains(&(p.0 + new_position.0, p.1 + new_position.1)))
                };

            let mut potential_new_position = position;

            let push = match push_iter.next() {
                Some(r) => r,
                None => {
                    push_iter = pushes.iter();
                    push_iter.next().unwrap()
                }
            };

            let x_extent = rock.x_extent();

            if *push == Push::Right {
                if position.0 + x_extent + 1 < 7 {
                    potential_new_position.0 += 1;
                }
            } else if position.0 > 0 {
                potential_new_position.0 -= 1;
            }

            // reset if it would intersect.
            if intersection(rock, &potential_new_position, &occupied) {
                potential_new_position = position;
            }

            potential_new_position.1 -= 1;
            if potential_new_position.1 == 0
                || intersection(rock, &potential_new_position, &occupied)
            {
                potential_new_position.1 = position.1;
                rock.positions.iter().for_each(|p| {
                    occupied.insert((
                        p.0 + potential_new_position.0,
                        p.1 + potential_new_position.1,
                    ));

                    max_y = std::cmp::max(max_y, p.1 + potential_new_position.1);
                });
                break;
            }

            position = potential_new_position;
        }
    }
    occupied.iter().map(|p| p.1).max().unwrap()
}

fn part2(pushes: &[Push], rocks: &[Rock]) -> i64 {
    let mut push_iter = pushes.iter();
    let mut rock_iter = rocks.iter();

    let mut push_idx = 0;
    let mut rock_idx = 0;

    let mut patterns = vec![];

    let mut heights = vec![];

    let mut occupied = HashSet::<(i64, i64)>::new();

    let mut max_y = 0;

    let mut maybe_cycle_len = 0;

    let mut cycle_index = 0;
    let mut cycle_detected = false;
    let mut start_cycle_index = 0;

    'outer: loop {
        let rock = match rock_iter.next() {
            Some(r) => r,
            None => {
                rock_iter = rocks.iter();
                rock_iter.next().unwrap()
            }
        };

        rock_idx += 1;
        rock_idx %= rocks.len();

        let starting_position = (2, max_y + 4);
        let mut position = starting_position;

        loop {
            let intersection =
                |rock: &Rock, new_position: &(i64, i64), occupied: &HashSet<(i64, i64)>| -> bool {
                    rock.positions
                        .iter()
                        .any(|p| occupied.contains(&(p.0 + new_position.0, p.1 + new_position.1)))
                };

            let mut potential_new_position = position;

            let push = match push_iter.next() {
                Some(r) => r,
                None => {
                    push_iter = pushes.iter();
                    push_iter.next().unwrap()
                }
            };

            push_idx += 1;
            push_idx %= pushes.len();

            let x_extent = rock.x_extent();

            if *push == Push::Right {
                if position.0 + x_extent + 1 < 7 {
                    potential_new_position.0 += 1;
                }
            } else if position.0 > 0 {
                potential_new_position.0 -= 1;
            }

            // reset if it would intersect.
            if intersection(rock, &potential_new_position, &occupied) {
                potential_new_position = position;
            }

            potential_new_position.1 -= 1;
            if potential_new_position.1 == 0
                || intersection(rock, &potential_new_position, &occupied)
            {
                potential_new_position.1 = position.1;
                rock.positions.iter().for_each(|p| {
                    occupied.insert((
                        p.0 + potential_new_position.0,
                        p.1 + potential_new_position.1,
                    ));

                    max_y = std::cmp::max(max_y, p.1 + potential_new_position.1);
                });

                let actual_rock_idx = if rock_idx == 0 {
                    rocks.len() - 1
                } else {
                    rock_idx - 1
                };
                let actual_push_idx = if push_idx == 0 {
                    pushes.len() - 1
                } else {
                    push_idx - 1
                };

                patterns.push((
                    actual_rock_idx,
                    actual_push_idx,
                    (
                        starting_position.0 - position.0,
                        starting_position.1 - position.1,
                    ),
                ));

                if !cycle_detected {
                    for cycle in cycle_index..patterns.len() - 1 {
                        if *patterns.last().unwrap() == patterns[cycle] {
                            cycle_index = cycle;
                            cycle_detected = true;
                            start_cycle_index = cycle_index;
                            maybe_cycle_len = patterns.len() - cycle_index;
                            break;
                        }
                    }
                } else if patterns[cycle_index + 1] == *patterns.last().unwrap() {
                    cycle_index += 1;
                    if cycle_index - start_cycle_index == maybe_cycle_len {
                        patterns.pop();
                        break 'outer;
                    }
                } else {
                    start_cycle_index = 0;
                    cycle_index = 0;
                    cycle_detected = false;
                }

                heights.push(max_y);
                break;
            }

            position = potential_new_position;
        }
    }

    maybe_cycle_len -= 1;
    let repeat_idx = patterns.len() - maybe_cycle_len;

    let height_offset = heights[repeat_idx] - heights[repeat_idx - 1];
    let baseline = heights[repeat_idx];
    let height = *heights.last().unwrap() - baseline;
    heights.iter_mut().for_each(|h| *h -= baseline);
    heights = heights
        .iter()
        .skip(repeat_idx)
        .copied()
        .collect::<Vec<_>>();

    let height = height + height_offset;
    let target = 1000000000000;

    let multiple = (target - 1 - repeat_idx) / (maybe_cycle_len);
    let remainder = (target - 1 - repeat_idx) % (maybe_cycle_len);
    (multiple as i64) * height + baseline + heights[remainder]
}

fn main() {
    let input = include_str!("../input.txt");

    let pushes = input
        .chars()
        .map(|c| match c {
            '<' => Push::Left,
            '>' => Push::Right,
            _ => panic!(),
        })
        .collect::<Vec<Push>>();

    let rocks = vec![
        Rock {
            positions: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        },
        Rock {
            positions: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        },
        Rock {
            positions: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        },
        Rock {
            positions: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        },
        Rock {
            positions: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        },
    ];

    println!("{}", part1(&pushes, &rocks, 2022));
    println!("{}", part2(&pushes, &rocks));
}
