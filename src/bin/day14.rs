use advent21::*;
use std::collections::HashMap;
use std::cmp;

// the one with inserting atoms into a molecule
fn main() {
    let inputs = load_inputs("day14").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// exact same thing except 40 steps instead of 10. I see this is another one
// designed to prank a naïve solution's performance.
fn part_two(inputs: &str) -> usize {
    part_every(inputs, 40)
}

// simultaneously insert the indicated element between each pair of elements. Do
// this 10 times, then subtract the least common element from the most common
// element.
// I would like to cheat in the following ways:
// - Re-use just two vecs and take advantage of their auto-resizing.
// - Do something funky with the "pairs" issue.
fn part_one(inputs: &str) -> usize {
    part_every(inputs, 10)
}

fn part_every(inputs: &str, iterations: usize) -> usize {
    let (template, rules) = parse_inputs(inputs);
    println!("the stuff is here.");
    // dbg!(&template);
    // dbg!(&rules);
    let mut polymer = template; // a move
    let mut spare: Vec<char> = Vec::new();

    // ok 3, 2, 1, lets jam
    for i in 0..iterations {
        dbg!(i);
        let previous = polymer; // a move
        polymer = spare; // a move
        polymer.clear();
        let length = previous.len();

        for i in 0..length {
            polymer.push('x');
            polymer.push('x');
        }
        // reduce reuse recycle
        spare = previous;
    }

    // I think that's our molecule:
    // println!("{:?}", &polymer);
    println!("{} chars long", polymer.len());
    let count = count_elements(polymer); // consumes polymer
    dbg!(&count);
    let max = count.iter().fold(0, |accum, (_, num)| cmp::max(accum, *num));
    let min = count.iter().fold(usize::MAX, |accum, (_, num)| cmp::min(accum, *num));
    let difference = max - min;
    println!("most common minus least common: {}", difference);

    difference
}

fn count_elements(polymer: Vec<char>) -> HashMap<char, usize> {
    let mut count: HashMap<char, usize> = HashMap::new();
    for ch in polymer {
        let num = count.entry(ch).or_insert(0);
        *num += 1;
    }
    count
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
        let answer = 2188189693529;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
