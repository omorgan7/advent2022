fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        input.lines().fold(0, |s, line| {
            let mut split = line.split(',');
            let first = split
                .next()
                .unwrap()
                .split('-')
                .map(|a| a.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let second = split
                .next()
                .unwrap()
                .split('-')
                .map(|a| a.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if first[0] <= second[0] && first[1] >= second[1]
                || second[0] <= first[0] && second[1] >= first[1]
            {
                s + 1
            } else {
                s
            }
        })
    );

    println!(
        "{}",
        input.lines().fold(0, |s, line| {
            let mut split = line.split(',');
            let first = split
                .next()
                .unwrap()
                .split('-')
                .map(|a| a.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let second = split
                .next()
                .unwrap()
                .split('-')
                .map(|a| a.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            if first
                .iter()
                .all(|a| *a < second[0] && *a < second[1] || *a > second[0] && *a > second[1])
                && second
                    .iter()
                    .all(|a| *a < first[0] && *a < first[1] || *a > first[0] && *a > first[1])
            {
                s
            } else {
                s + 1
            }
        })
    );
}
