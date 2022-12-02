enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Score {
    Win,
    Draw,
    Loss,
}

impl Play {
    fn new(c: u8) -> Play {
        if c == b'A' || c == b'X' {
            Play::Rock
        } else if c == b'B' || c == b'Y' {
            Play::Paper
        } else {
            Play::Scissors
        }
    }

    fn player_score(&self) -> i64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl Score {
    fn new(c: u8) -> Score {
        if c == b'X' {
            Score::Loss
        } else if c == b'Y' {
            Score::Draw
        } else {
            Score::Win
        }
    }
}

fn score_pt1(a: &Play, b: &Play) -> i64 {
    match a {
        Play::Rock => match b {
            Play::Rock => 3,
            Play::Paper => 6,
            Play::Scissors => 0,
        },
        Play::Paper => match b {
            Play::Rock => 0,
            Play::Paper => 3,
            Play::Scissors => 6,
        },
        Play::Scissors => match b {
            Play::Rock => 6,
            Play::Paper => 0,
            Play::Scissors => 3,
        },
    }
}

fn score_pt2(a: &Play, b: &Score) -> i64 {
    match a {
        Play::Rock => match b {
            Score::Win => 6 + Play::Paper.player_score(),
            Score::Loss => Play::Scissors.player_score(),
            Score::Draw => 3 + Play::Rock.player_score(),
        },
        Play::Paper => match b {
            Score::Win => 6 + Play::Scissors.player_score(),
            Score::Loss => Play::Rock.player_score(),
            Score::Draw => 3 + Play::Paper.player_score(),
        },
        Play::Scissors => match b {
            Score::Win => 6 + Play::Rock.player_score(),
            Score::Loss => Play::Paper.player_score(),
            Score::Draw => 3 + Play::Scissors.player_score(),
        },
    }
}

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "{}",
        input.lines().fold(0, |s, line| {
            let line_bytes = line.as_bytes();
            let opponent = Play::new(line_bytes[0]);
            let me = Play::new(line_bytes[2]);

            s + score_pt1(&opponent, &me) + me.player_score()
        })
    );

    println!(
        "{}",
        input.lines().fold(0, |s, line| {
            let line_bytes = line.as_bytes();
            let opponent = Play::new(line_bytes[0]);
            let me = Score::new(line_bytes[2]);

            s + score_pt2(&opponent, &me)
        })
    );
}
