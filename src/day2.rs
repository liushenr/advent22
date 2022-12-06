use std::{cmp::Ordering, str::FromStr, string::ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]

enum Score {
    Rock,
    Paper,
    Scissors,
}
//compare the win by ordering
impl PartialOrd for Score {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> std::option::Option<std::cmp::Ordering> {
        Some(if self == other {
            Ordering::Equal
        } else if self.greater() == *other {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl Score {
    //return score for the shape
    fn score(&self) -> usize {
        match self {
            Score::Rock => 1,
            Score::Paper => 2,
            Score::Scissors => 3,
        }
    }
    //return the shape greater than self
    fn greater(&self) -> Self {
        match self{
            Score::Rock => Self::Paper,
            Score::Paper => Self::Scissors,
            Score::Scissors => Self::Rock,
        }

    }
    //return the shape less than self
    fn less(&self) -> Self {
        match self {
            Score::Rock => Self::Scissors,
            Score::Paper => Self::Rock,
            Score::Scissors => Self::Paper,
        }
    }
}
impl FromStr for Score {
    type Err = ParseError;

    //take &str match to shape
    fn from_str(s:&str) -> Result<Self, Self::Err> {
        match s {
            "A"|"X" => Ok(Self::Rock),
            "B"|"Y" => Ok(Self::Paper),
            "C"|"Z" => Ok(Self::Scissors),
            _ => todo!(),
        }
    }

}
//calculate the score for the round
fn score(our: Score, their: Score) -> (usize, usize) {
    let (o, t) = if our == their {
            (3, 3)
        } else if our < their {
            (0, 6)
        } else {
            (6, 0)
        };
        (our.score() + o, their.score() + t)
}
enum Outcome {
    Win,
    Lose,
    Draw
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s:&str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _=> todo!(),
        }
    }
}
fn part1(s: &str) -> usize {
    let mut total = 0;
    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Score::from_str(sp[0]).unwrap();
        let ours = Score::from_str(sp[1]).unwrap();
        let score = score(ours, theirs);
        //sum our score
        total += score.0;
    }
    total
}

fn part2(s: &str) -> usize {
    let mut total = 0;

    for line in s.lines() {
        let sp: Vec<_> = line.split_ascii_whitespace().collect();
        let theirs = Score::from_str(sp[0]).unwrap();
        let outcome = Outcome::from_str(sp[1]).unwrap();
        let ours = match(theirs, outcome) {
            (t, Outcome::Draw) => t,
            (t, Outcome::Win) => t.greater(),
            (t, Outcome::Lose) => t.less(),
        };
        let score = score(ours, theirs);
        total += score.0;
    }
    total
}

const SAMPLE: &str = "
";

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let s = load_input();
    let p1 = part1(&s);
    let p2 = part2(&s);
    println!("{p1}");
    println!("{p2}");
}
