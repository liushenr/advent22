use std::{str::FromStr, string::ParseError};
use std::cmp;


const SAMPLE: &str = "";

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    //contains check if self contains other
    fn contains(&self, other:&Self) -> bool {
        self.start <= other.start && self.end >= other.end 
    }

    fn overlaps(&self, other:&Self) -> bool {
        //min(two starts) <= max(two ends)
        cmp::max(self.start, other.start) <= cmp::min(self.end, other.end)
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<_> = s.split('-').flat_map(usize::from_str).collect();
        let [start, end] = &pair[..] else { todo!()};
        Ok(Self {
            start: *start,
            end: *end,
        })
    }
}

fn inner(s: &str, f: fn(&Range, &Range) -> bool) -> usize {
    s.lines()
        .map(|line| {
            let rs = line.split(',').flat_map(Range::from_str);
            let [a, b] = &rs.collect::<Vec<_>>()[..] else { todo!() };
            (f(a, b) || f(b, a)) as usize
        })
        .sum()
}

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let s = load_input();
    let p1 = inner(&s, Range::contains);
    let p2 = inner(&s, Range::overlaps);

    println!("the answer to part 1 is {p1}");
    println!("the answer to part 2 is {p2}");
}
