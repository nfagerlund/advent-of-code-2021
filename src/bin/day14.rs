use advent21::*;
use std::collections::HashMap;

// the one with inserting atoms into a molecule
fn main() {
    let inputs = load_inputs("day14").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// simultaneously insert the indicated element between each pair of elements. Do
// this 10 times, then subtract the least common element from the most common
// element.
// I would like to cheat in the following ways:
// - Re-use just two vecs and take advantage of their auto-resizing.
// - Do something funky with the "pairs" issue.
fn part_one(inputs: &str) -> usize {
    let (template, rules) = parse_inputs(inputs);
    println!("the stuff is here:");
    dbg!(&template);
    dbg!(&rules);

    0
}

fn parse_inputs(inputs: &str) -> (Vec<char>, RulesDict) {
    let (template, rules) = inputs.split_once("\n\n").unwrap();
    (parse_polymer_template(template), parse_insertion_rules(rules))
}

type RulesDict = HashMap<char, HashMap<char, char>>;

// rules[first][second] = insertion
fn parse_insertion_rules(inputs: &str) -> RulesDict {
    let mut rules: RulesDict  = HashMap::new();

    for line in inputs.lines() {
        let (pair, insertion) = line.split_once(" -> ").unwrap();
        let first = pair.chars().next().unwrap();
        let second = pair.chars().last().unwrap();
        let insertion = insertion.chars().next().unwrap();
        rules.entry(first).or_insert(HashMap::new()).insert(second, insertion);
    }

    rules
}

fn parse_polymer_template(inputs: &str) -> Vec<char> {
    inputs.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#;

    #[test]
    fn example_part_one() {
        let answer = 1588;
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
