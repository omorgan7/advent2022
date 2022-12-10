fn render(crt_x: &mut i64, crt_y: &mut i64, x: i64) {
    if *crt_x == x || *crt_x == x - 1 || *crt_x == x + 1 {
        print!("#")
    } else {
        print!(".")
    }
    *crt_x += 1;
    if *crt_x == 40 {
        *crt_x = 0;
        *crt_y += 1;
        println!();
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut x = 1;
    let mut cycle_count = 1;

    println!(
        "{}",
        input
            .lines()
            .filter_map(|line| {
                let mut split = line.split(' ');

                match split.next().unwrap() {
                    "addx" => {
                        cycle_count += 1;
                        let mut to_return: Option<i64> = None;
                        if (cycle_count - 20) % 40 == 0 {
                            to_return = Some(cycle_count * x);
                        }

                        x += split.next().unwrap().parse::<i64>().unwrap();

                        cycle_count += 1;
                        if to_return.is_none() && (cycle_count - 20) % 40 == 0 {
                            to_return = Some(cycle_count * x);
                        }

                        if to_return.is_some() {
                            return to_return;
                        } else {
                            return None;
                        }
                    }
                    "noop" => {
                        cycle_count += 1;
                        if (cycle_count - 20) % 40 == 0 {
                            return Some(cycle_count * x);
                        }
                    }
                    _ => {}
                }
                None
            })
            .sum::<i64>()
    );

    x = 1;
    cycle_count = 1;

    let mut crt_x = 0;
    let mut crt_y = 0;

    input.lines().for_each(|line| {
        let mut split = line.split(' ');

        match split.next().unwrap() {
            "addx" => {
                render(&mut crt_x, &mut crt_y, x);
                cycle_count += 1;

                render(&mut crt_x, &mut crt_y, x);
                x += split.next().unwrap().parse::<i64>().unwrap();
                cycle_count += 1;
            }
            "noop" => {
                cycle_count += 1;
                render(&mut crt_x, &mut crt_y, x);
            }
            _ => {}
        }
    });
}
