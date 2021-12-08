use std::fs;

fn load_inputs(dataset: &str) -> std::io::Result<String> {
    let file = format!("./inputs/{}.txt", dataset);
    fs::read_to_string(file)
}

fn main() {
    let inputs = load_inputs("day1a").unwrap();
    let first = inputs.lines().next().unwrap();
    println!("{}", first);
}
