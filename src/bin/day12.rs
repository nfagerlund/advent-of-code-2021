use advent21::*;
use std::collections::HashMap;

// The one with traversing a cave graph.
fn main() {
    let inputs = load_inputs("day12").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> usize {
    let edges = parse_inputs_tentatively(inputs);
    println!("The stuff is here: {:#?}", &edges);
    0
}

// Wow I have no idea how to do this one. I guess my first guess at a useful
// structure for the inputs is a hash of vecs?
fn parse_inputs_tentatively<'a>(inputs: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    // n.b. I don't know if I actually need those lifetimes up top. Investigate later.
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in inputs.lines() {
        let (source, dest) = line.split_once('-').unwrap();
        edges.entry(source).or_insert(Vec::new()).push(dest);
        edges.entry(dest).or_insert(Vec::new()).push(source);
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    #[test]
    fn example_part_one() {
        let answer = 226;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
