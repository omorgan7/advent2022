use std::vec::Vec;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: (usize, usize),
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
struct Edge {
    cost: i64,
    position: (usize, usize),
}

struct Grid {
    width: usize,
    heightmap: Vec<i64>,
}

impl Grid {
    fn at(&self, x: i64, y: i64) -> i64 {
        self.heightmap[(x + self.width as i64 * y) as usize]
    }
}

fn part1(graph: &[Vec<Edge>], width: usize, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| i64::MAX).collect();

    let mut parents: Vec<(usize, usize)> =
        (0..graph.len()).map(|_| (usize::MAX, usize::MAX)).collect();

    let mut heap = BinaryHeap::new();

    let to_position = |p: (usize, usize)| -> usize { p.0 + p.1 * width };

    dist[to_position(start)] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    let mut path = Vec::<(usize, usize)>::new();

    while let Some(State { cost, position }) = heap.pop() {
        path.push(position);
        if position == end {
            break;
        }

        if cost > dist[to_position(position)] {
            path.pop();
            continue;
        }

        for edge in &graph[to_position(position)] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.position,
            };

            if next.cost < dist[to_position(next.position)] {
                heap.push(next);
                dist[to_position(next.position)] = next.cost;
                parents[to_position(next.position)] = (position.0, position.1);
            }
        }
    }

    let mut count = 0;

    let mut parent = end;

    while parent != start && parent != (usize::MAX, usize::MAX) {
        count += 1;
        parent = parents[to_position(parent)];
    }
    if parent == (usize::MAX, usize::MAX) {
        i64::MAX
    } else {
        count
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let heightmap = input
        .lines()
        .flat_map(|line| {
            line.as_bytes().iter().map(|c| match c {
                b'S' => 0,
                b'E' => 27,
                _ => (c - b'a' + 1) as i64,
            })
        })
        .collect::<Vec<i64>>();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if heightmap[x + width * y] == 0 {
                start = (x, y);
            }
            if heightmap[x + width * y] == 27 {
                end = (x, y);
            }
        }
    }

    let mut grid = Grid {
        width,
        heightmap,
    };

    let mut graph = Vec::<Vec<Edge>>::new();

    for y in 0..height as i64 {
        for x in 0..width as i64 {
            let mut edges = Vec::<Edge>::new();

            let h = height as i64;
            let w = width as i64;

            let neighbours = if x == 0 && y == 0 {
                [(0, 1), (1, 0), (0, 0), (0, 0)]
            } else if x == w - 1 && y == 0 {
                [(-1, 0), (0, 1), (0, 0), (0, 0)]
            } else if x == 0 && y == h - 1 {
                [(1, 0), (0, -1), (0, 0), (0, 0)]
            } else if x == w - 1 && y == h - 1 {
                [(-1, 0), (0, -1), (0, 0), (0, 0)]
            } else if x == 0 {
                [(1, 0), (0, -1), (0, 1), (0, 0)]
            } else if y == 0 {
                [(0, 1), (1, 0), (-1, 0), (0, 0)]
            } else if x == w - 1 {
                [(0, 1), (0, -1), (-1, 0), (0, 0)]
            } else if y == h - 1 {
                [(1, 0), (-1, 0), (0, -1), (0, 0)]
            } else {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
            };

            for n in neighbours {
                let nn = (x + n.0, y + n.1);
                let diff = grid.at(nn.0, nn.1) - grid.at(x, y);
                if diff == 1 || (diff == 0 && (x, y) != nn) || diff < 0 {
                    edges.push(Edge {
                        cost: if diff < 0 { 1 } else { diff + 1 },
                        position: (nn.0 as usize, nn.1 as usize),
                    });
                }
            }

            graph.push(edges);
        }
    }

    println!("{}", part1(&graph, width, start, end));

    grid.heightmap[grid.width * start.1 + start.0] = 1;

    println!(
        "{}",
        grid.heightmap
            .iter()
            .enumerate()
            .filter_map(|(idx, h)| {
                if *h == 1 {
                    Some(((idx % grid.width), (idx / grid.width)))
                } else {
                    None
                }
            })
            .map(|s| part1(&graph, width, s, end))
            .min()
            .unwrap()
    );
}
