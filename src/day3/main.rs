use std::collections::{hash_map::RandomState, HashMap, HashSet};

const SAMPLE: &str = "";

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn priority(b: u8) -> usize {
    //based on b is lower or upper case, assign priorities
    if (b'A'..=b'Z').contains(&b) {
        (b - b'A' + 27) as usize
    } else {
        (b -b'a' +1) as usize
    }
}

fn part1(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let(a, b) = line.split_at(line.len()/2);
            //find the position of the same char in a and b
            let i = a.find(&b.chars().collect::<Vec<_>>()[..]).unwrap();
            priority(a.as_bytes()[i])
        })
        .sum()
}

fn part2(s: &str) -> usize {
    s.lines()
        //read each line to bytes
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        //divide to group of 3
        .chunks(3)
        .map(|s| {
            let [a, b, c] = s else { todo!() };

            //find intersectionb etween a and b, then ab and c
            let ab: HashSet<u8, RandomState> =
                HashSet::from_iter(a.intersection(b).cloned());
            priority(*ab.intersection(c).next().unwrap())
        })
        .sum()
}

fn main() {
     let s = load_input();
     let p1 = part1(&s);
     let p2 = part2(&s);
     println!("the answert to part1 is {p1}.");
     println!("the answert to part2 is {p2}.");
}
