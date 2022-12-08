use std::collections::HashSet;


const SAMPLE: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn part1(s: &str) -> usize {
    let v: Vec<_> = s.chars().collect();
    let mut count = 4;
    for w in v.windows(4) {
        let h: HashSet<_> = w.iter().collect();
        if h.len() == 4 {
            break;
        }
        count += 1;
    }
    return count;
}

fn part2(s: &str) -> usize {
    let v: Vec<_> = s.chars().collect();
    let mut count = 14;
    for w in v.windows(14) {
        let h: HashSet<_> = w.iter().collect();
        if h.len() == 14 {
            break;
        }
        count += 1;
    }
    return count;
}

fn main() {
    let s = load_input();
    let p1 = part1(&s);
    let p2 = part2(&s);
    println!("the answert to part1 is {p1}.");
    println!("the answert to part2 is {p2}.");
}
