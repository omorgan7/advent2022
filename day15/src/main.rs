use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn distance(&self, other: &Vector2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn part2(input: &[(Vector2, Vector2)]) -> i64 {
    let mut found_x = -1;
    let mut found_y = -1;

    let bounds = 4000000;
    // let bounds = 20;

    'outer: for (sensor, beacon) in input {
        let distance = sensor.distance(beacon);

        let x_start = sensor.x - distance - 1;
        let y_start = sensor.y;

        let mut x_direction = 1;
        let mut y_direction = 1;

        let mut x = x_start;
        let mut y = y_start;

        loop {
            if x >= 0
                && x <= bounds
                && y >= 0
                && y <= bounds
                && input.iter().all(|(sensor2, beacon2)| {
                    let beacon_distance = sensor2.distance(beacon2);
                    let point_distance = sensor2.distance(&Vector2 { x, y });

                    point_distance > beacon_distance
                })
            {
                found_x = x;
                found_y = y;
                break 'outer;
            }

            x += x_direction;
            y += y_direction;

            if x == x_start && y == y_start {
                break;
            }

            if x == sensor.x + distance + 1 || x == sensor.x - distance - 1 {
                x_direction *= -1;
            }
            if y == sensor.y + distance + 1 || y == sensor.y - distance - 1 {
                y_direction *= -1;
            }
        }
    }

    found_x * 4000000 + found_y
}

fn part1(input: &[(Vector2, Vector2)]) -> i64 {
    let mut no_beacons = HashMap::<i64, Vec<Vector2>>::new();

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;

    for (sensor, beacon) in input {
        let distance = sensor.distance(beacon);

        min_x = std::cmp::min(min_x, sensor.x - distance);
        max_x = std::cmp::max(max_x, sensor.x + distance);
    }

    let y_coord = 2000000;
    for x in min_x..=max_x {
        if input.iter().all(|(sensor, beacon)| {
            let beacon_distance = sensor.distance(beacon);
            let point_distance = sensor.distance(&Vector2 { x, y: y_coord });

            point_distance > beacon_distance
        }) {
            no_beacons
                .entry(y_coord)
                .or_insert(vec![])
                .push(Vector2 { x, y: y_coord });
        }
    }

    (max_x - min_x) - no_beacons.get(&y_coord).unwrap().len() as i64
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let delim = line.find(':').unwrap();
            let mut start = line.to_string();
            let end = start.split_off(delim);

            let mut sensor_split = start[12..].split(", ");

            let sensor_x = sensor_split.next().unwrap().parse::<i64>().unwrap();
            let sensor_y = sensor_split.next().unwrap()[2..].parse::<i64>().unwrap();

            let mut beacon_split = end[25..].split(", ");
            let beacon_x = beacon_split.next().unwrap().parse::<i64>().unwrap();
            let beacon_y = beacon_split.next().unwrap()[2..].parse::<i64>().unwrap();

            (
                Vector2 {
                    x: sensor_x,
                    y: sensor_y,
                },
                Vector2 {
                    x: beacon_x,
                    y: beacon_y,
                },
            )
        })
        .collect::<Vec<(Vector2, Vector2)>>();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
