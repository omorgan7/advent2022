use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{BTreeMap, VecDeque};
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

fn part1_recursive(
    current: usize,
    visited: &Vec<(i64, i64)>,
    flows: &[i64],
    map: &[i64],
    count: usize,
    minutes_remaining: i64,
) -> (i64, Vec<(i64, i64)>) {
    if minutes_remaining <= 0 {
        return (
            visited
                .iter()
                .map(|(n, minute)| flows[*n as usize] * minute)
                .sum(),
            visited.clone(),
        );
    }

    let neighbours = map[count * current..count * (current + 1)]
        .iter()
        .enumerate()
        .filter(|(idx, m)| {
            (minutes_remaining - *m - 1) * flows[*idx] > 0
                && !visited.iter().any(|(node, _)| *node == (*idx as i64))
        })
        .map(|(idx, _)| idx);

    let mut best = 0;
    let mut best_visited = vec![];

    for n in neighbours {
        let mut new_visited = visited.clone();
        new_visited.push((n as i64, minutes_remaining - map[count * current + n] - 1));

        let (maybe_best, new_v) = part1_recursive(
            n,
            &new_visited,
            flows,
            map,
            count,
            minutes_remaining - map[count * current + n] - 1,
        );

        if maybe_best > best {
            best = maybe_best;
            best_visited = new_v.clone();
        }
    }

    let v: &Vec<(i64, i64)> = if best_visited.is_empty() {
        visited
    } else {
        &best_visited
    };
    (
        v.iter()
            .map(|(n, minute)| flows[*n as usize] * minute)
            .sum(),
        v.clone(),
    )
}

fn part1(labels: &[String], flows: &[i64], map: &[i64], count: usize) -> i64 {
    let minutes_remaining = 30;
    let opened = vec![];

    part1_recursive(
        labels
            .iter()
            .enumerate()
            .find(|(_, l)| *l == "AA")
            .unwrap()
            .0,
        &opened,
        flows,
        map,
        count,
        minutes_remaining,
    )
    .0
}

fn part2(labels: &[String], flows: &[i64], map: &[i64], count: usize) -> i64 {
    let minutes_remaining = 26;

    let start = labels
        .iter()
        .enumerate()
        .find(|(_, l)| *l == "AA")
        .unwrap()
        .0;

    let mut queue = VecDeque::new();

    let mut best = BTreeMap::<Vec<usize>, i64>::new();
    queue.push_back((minutes_remaining, start, 0, Vec::<usize>::new()));

    while let Some((remaining, node, relief, visited)) = queue.pop_front() {
        map[count * node..count * (node + 1)]
            .iter()
            .enumerate()
            .filter(|(idx, m)| (remaining - *m - 1) * flows[*idx] > 0 && !visited.contains(idx))
            .map(|(idx, _)| idx)
            .for_each(|i| {
                let mut new_visited = visited.clone();
                new_visited.push(i);
                let time = remaining - map[count * node + i] - 1;
                queue.push_back((time, i, relief + time * flows[i], new_visited.clone()));
                if best.contains_key(&new_visited) {
                    let old = *best.get(&new_visited).unwrap();
                    *best.get_mut(&new_visited).unwrap() =
                        std::cmp::max(old, relief + time * flows[i]);
                } else {
                    best.insert(new_visited, relief + time * flows[i]);
                }
            });
    }

    let mut best_flow = 0;
    for (p0, f0) in &best {
        for (p1, f1) in &best {
            if p0.iter().any(|i| p1.contains(i)) {
                continue;
            }

            best_flow = std::cmp::max(f0 + f1, best_flow);
        }
    }

    best_flow
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
}

fn main() {
    let input = include_str!("../input.txt");

    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)")
        .unwrap();

    let valve_count = input.lines().count();

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
                tunnels[valve_count * edge + idx] = 1;
                tunnels[valve_count * idx + edge] = 1;
            }
        });
    }

    dijkstra(&mut tunnels, valve_count);

    println!("{}", part1(&labels, &flows, &tunnels, valve_count));
    println!("{}", part2(&labels, &flows, &tunnels, valve_count));
}
