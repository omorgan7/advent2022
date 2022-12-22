use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Valve {
    label: String,
    flow_rate: i64,
    tunnels: Vec<Rc<RefCell<Valve>>>,
}

fn part1(labels: &[String], flows: &[i64], map: &[i64], count: usize) -> i64 {
    let mut minutes_remaining = 30;

    let mut i = 0;

    let mut opened = vec![];

    while minutes_remaining > 0 {
        let energy = map[i * count..(i + 1) * count]
            .iter()
            .enumerate()
            .map(|(idx, dist)| {
                if opened.contains(&idx) {
                    0
                } else {
                    flows[idx] * (minutes_remaining - dist - 1)
                }
            })
            .collect::<Vec<i64>>();

        println!("{:?}", energy);
        println!("--------------------");

        let total_energy: i64 = energy.iter().sum();

        let mut best = i64::MIN;
        let mut best_idx = usize::MAX;

        for (i0, e) in energy.iter().enumerate() {
            if flows[i0] == 0 {
                continue;
            }

            println!(
                "{:?}",
                map[i * count..(i + 1) * count]
                    .iter()
                    .enumerate()
                    .map(|(i1, dist)| {
                        if opened.contains(&i1) || i0 == i1 {
                            0
                        } else {
                            flows[i1] * (minutes_remaining - map[i0 + i1 * count] - dist - 1)
                        }
                    })
                    .collect::<Vec<i64>>()
            );

            let hypothetical_best_energy = map[i * count..(i + 1) * count]
                .iter()
                .enumerate()
                .map(|(i1, dist)| {
                    if opened.contains(&i1) || i1 == i0 {
                        0
                    } else {
                        flows[i1] * (minutes_remaining - map[i0 + i1 * count] - dist - 1)
                    }
                })
                .sum::<i64>();

            if hypothetical_best_energy > best {
                best_idx = i0;
                best = hypothetical_best_energy;
            }
        }

        println!("Opening: {}", best_idx);
        if best_idx == usize::MAX {
            break;
        }

        opened.push(best_idx);
        minutes_remaining -= map[i * count + best_idx];
        i = best_idx;
    }

    print!("Opened: ");
    for o in opened {
        print!("{} ", o);
    }
    println!();
    0
}

fn dijkstra(map: &mut [i64], mapcount: usize) {
    for start in 0..mapcount {
        for end in 0..mapcount {
            if start == end {
                continue;
            }

            if map[start * mapcount + end] != 0 {
                continue;
            }

            let mut heap = BinaryHeap::new();
            let mut dist: Vec<_> = (0..mapcount).map(|_| i64::MAX).collect();
            dist[start] = 0;
            heap.push(State {
                cost: 0,
                position: start,
            });

            while let Some(State { cost, position }) = heap.pop() {
                if position == end {
                    break;
                }

                if cost > dist[position] {
                    continue;
                }

                for (idx, edge) in map[mapcount * position..mapcount * (position + 1)]
                    .iter()
                    .enumerate()
                {
                    if *edge == 0 {
                        continue;
                    }

                    let next = State {
                        cost: cost + edge,
                        position: idx,
                    };

                    if next.cost < dist[next.position] {
                        heap.push(next);
                        dist[next.position] = next.cost;
                    }
                }
            }

            map[start * mapcount + end] = dist[end];
            map[end * mapcount + start] = dist[end];
        }
    }

    for y in 0..mapcount {
        for x in 0..mapcount {
            if map[x + mapcount * y] < 10 {
                print!(" ");
            }
            print!("{} ", map[x + mapcount * y]);
        }
        println!();
    }
    println!();
}

fn main() {
    let input = include_str!("../input.txt");

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
        .unwrap();

    let mut valve_count = input.lines().count();

    let mut labels = vec![];
    let mut flows = vec![];
    let mut tunnels = vec![0i64; valve_count * valve_count];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();

        labels.push(captures.get(1).unwrap().as_str().to_string());

        flows.push(captures.get(2).unwrap().as_str().parse::<i64>().unwrap());

        let this_tunnels = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|t| t.to_string());

        let idx = labels.len() - 1;

        this_tunnels.for_each(|tunnel| {
            if let Some(edge) = labels.iter().position(|label| *label == tunnel) {
                println!("Found {} for {} at index: {}", tunnel, labels[idx], edge);
                tunnels[valve_count * edge + idx] = 1;
                tunnels[valve_count * idx + edge] = 1;
            }
        });
    }

    dijkstra(&mut tunnels, valve_count);

    println!("{}", part1(&labels, &flows, &tunnels, valve_count));

    // print!("  ");
    // for label in &labels {
    //     print!("{} ", label.chars().next().unwrap());
    // }
    // println!();
    // for y in 0..valve_count {
    //     print!("{} ", &labels[y].chars().next().unwrap());
    //     for x in 0..valve_count {
    //         print!("{} ", tunnels[x + valve_count * y]);
    //     }
    //     println!();
    // }
    // println!("{:?}", tunnels);

    // for v in &valves {
    //     let vv = v.borrow();
    //     println!(
    //         "Valve: {{ label: {}, flow_rate: {}, tunnels: {:?} }}",
    //         vv.label,
    //         vv.flow_rate,
    //         vv.tunnels
    //             .iter()
    //             .map(|t| t.borrow().label.to_string())
    //             .collect::<Vec<String>>()
    //     );
    // }

    // println!("{}", part1(Rc::clone(&valves[0]), None, vec![], 0, 0));
}
