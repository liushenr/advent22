const SAMPLE: &str = "
";

fn load_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn main() {
    let input = load_input();
    let mut elves = p1(&input);

    //sort elves value desc
    elves.sort();
    elves.reverse();

    //adding the first three
    dbg!(elves[..3].iter().sum::<usize>());
    //println!("{}", input);
}

fn p1(inp: &str) -> Vec<usize> {
        //a vector to hold elves calories sum
        let mut elves = Vec::new();

        //hold running total
        let mut sum = 0;

        for line in inp.lines() {
            if line.is_empty() {
                elves.push(sum);
                sum = 0;
            } else {
                sum += line.parse::<usize>().unwrap();
            }
        }
    elves.push(sum);
    println!("max = {}", elves.iter().max().unwrap());
    elves
}
