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
    let system = parse_inputs_tentatively(inputs);
    // println!("The stuff is here: {:#?}", &system);
    let shotgun_shack: Vec<&str> = Vec::new();
    let paths = maybe_traverse_caves("start", &shotgun_shack, &system);
    // I want to see it, and want fewer lines than dbg! gets me.
    let dumpage: Vec<String> = paths.iter().map(|v| format!("{:?}", v)).collect();
    let dumpage = dumpage.join("\n");
    println!("{}", &dumpage);

    let count = paths.len();
    println!("Unique paths to end: {}", count);
    count
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

// ok let's be real dumb about these, don't even scan the whole string
fn is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_ascii_lowercase()
}
fn is_large(cave: &str) -> bool {
    cave.chars().next().unwrap().is_ascii_uppercase()
}

// Right, so, I think the hardest-seeming part to me right now is the allowed
// backtracking. How do I know when to stop going back to a big cave? Well, got
// to keep track somehow. Also... what do I do when I've gotten into a dead end?
// oh. return blank.

// Idk if I need this.
fn is_exhausted(cave: &str, visited: &Vec<&str>, system: &HashMap<&str, Vec<&str>>) -> bool {
    if is_small(cave) && visited.contains(&cave) {
        true
    } else {
        let edges = system.get(cave).unwrap();
        // if we can go anywhere non-exhausted from here...
        if edges.iter().any(|c| !is_exhausted(c, visited, system)) {
            false
        } else {
            true
        }
    }
}

// Ok, ok. So, I think I can do this with a recursive for-loop. It's just that the contents of the loop are likely goofy.

// how_did_i_get_here is the path before we got to the current cave (well). This
// doesn't expect that to get appended before it's called.
fn maybe_traverse_caves<'a>(well: &'a str, how_did_i_get_here: &Vec<&'a str>, system: &HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    // ^^ arg how_did_i_get_here is likely in use by other branches of the
    // recursion, so I don't want to take it as mutable.

    // Last things first:
    // - if we're at a dead-end, return failure. A dead-end is a small cave
    //   other than "end" that we've been to before. This might even be "start"!
    //   For now I'll avoid prematurely optimizing these backtracks.
    if is_small(well) && how_did_i_get_here.contains(&well) {
        return vec![];
    }
    // prep for next branches, which will need the current cave
    let mut into_the_silent_water: Vec<&str> = how_did_i_get_here.clone();
    into_the_silent_water.push(well);
    // - if we're at the end, return success (1 path).
    if well == "end" {
        return vec![into_the_silent_water];
    }
    // - ok, we're traversing! we're recursing! return ambiguous result (many paths)
    let destinations = system.get(well).unwrap();
    let results: Vec<Vec<&str>> = destinations.iter()
        .map(|cave| maybe_traverse_caves(cave, &into_the_silent_water, system))
        .flatten()
        .filter(|path| path.len() > 0)
        .collect();
    results
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
