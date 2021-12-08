use std::fs;

fn main() {
    let inputs = fs::read_to_string("./inputs/day1a.txt").unwrap();
    let first = inputs.lines().next().unwrap();
    println!("{}", first);
}
