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
    let (template, rules) = inputs.split_once("\n\n").unwrap();
    // let template = parse_polymer_template(template);
    // On further consideration, do NOT parse the polymer template further. Just use the &str.
    let rules = parse_insertion_rules_for_cheaters(rules);
    // println!("The stuff is here:\n{:#?}", &rules);

    let first_element = template.chars().next().unwrap();
    let last_element = template.chars().last().unwrap();
    let mut pair_counts: HashMap<&str, usize> = HashMap::new();
    let mut pair_scratchpad: HashMap<&str, usize> = HashMap::new();

    // OK, populate initial pair counts
    for i in 0..template.len() {
        if i + 1 < template.len() {
            // We know this is ascii, so we're fine to go slicing.
            let pair = &template[i..=(i + 1)];
            let count_entry = pair_counts.entry(pair).or_insert(0);
            *count_entry += 1;
        }
    }
    println!("The other stuff is here: \n{:#?}", &pair_counts);

    0
}

fn part_every_unusable(inputs: &str, iterations: usize) -> usize {
    let (template, rules) = parse_inputs(inputs);
    println!("the stuff is here.");
    // dbg!(&template);
    // dbg!(&rules);
    let mut polymer = template; // a move
    let mut spare: Vec<char> = Vec::new();

    // ok 3, 2, 1, lets jam
    for _ in 0..iterations {
        let previous = polymer; // a move
        polymer = spare; // a move
        polymer.clear();
        let length = previous.len();

        for i in 0..length {
            let current = previous[i];
            polymer.push(current);
            let next_index = i + 1;
            if next_index < length {
                let next = previous[next_index];
                let insertion = rules.get(&current).unwrap().get(&next).unwrap();
                polymer.push(*insertion);
            }
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

// Ok, we're going to be increasing counters, for this one. That means we need
// to get back keys to know which two pair counters to increase for a given
// source pair, and we'll end up increasing them by the amount of that source
// pair we have. I figure we're storing the counters in a HashMap<&str, usize>.
// We'll need a WIP hashmap to tally things up in before we combine them at the
// end of the step, I think, since these are all meant to happen simultaneously.
// Oh, wait, if I'm gonna borrow a &str for the hash keys, I need to borrow it
// from somewhere -- it's not gonna exist verbatim in the inputs...? No wait, it
// will, because the inputs have every pair we'll ever see. It's just.. might be
// tricky to grab it when we want it. Okay, sure, let's do some weird borrowing
// gymnastics.
fn parse_insertion_rules_for_cheaters(inputs: &str) -> HashMap<&str, (&str, &str)> {
    let mut wip_rules: HashMap<&str, Option<(&str, &str)>> = HashMap::new();
    // first pass: set all to none
    for line in inputs.lines() {
        let (pair, _) = line.split_once(" -> ").unwrap();
        wip_rules.insert(pair, None);
    }
    // OK, now all the keys are slices from inputs! but if we call .insert again,
    // it won't update the keys to be a new slice, it'll just do == on em! So we
    // can construct keys willy-nilly without needing to keep their source
    // alive.
    // And!! we can use .entry().key() to get an original reference! &&str I think.
    for line in inputs.lines() {
        let (pair, insertion) = line.split_once(" -> ").unwrap();

        let p1 = pair.chars().next().unwrap();
        let mut p1 = String::from(p1);
        p1.push_str(insertion);
        let (p1_laundered, _) = wip_rules.get_key_value(&p1[..]).unwrap();
        let p1_laundered = *p1_laundered;

        let mut p2 = String::from(insertion);
        p2.push(pair.chars().last().unwrap());
        let (p2_laundered, _) = wip_rules.get_key_value(&p2[..]).unwrap();
        let p2_laundered = *p2_laundered;

        // lol nasty

        // ok...
        wip_rules.insert(pair, Some((p1_laundered, p2_laundered)));
    }

    let mut rules: HashMap<&str, (&str, &str)> = HashMap::new();
    // ok...
    for (k, v) in wip_rules.drain() {
        rules.insert(k, v.unwrap());
    }
    rules
}

fn parse_polymer_template(inputs: &str) -> Vec<char> {
    inputs.chars().collect()
}

// polymer analysis
// NNCB
// NN NC CB (pair counts for above)
// NCNBCHB (real result)
// NC CN NB BC CH HB (new pair counts: 6)
//     (OH! okay, um,)
// NBC CCN NBB BBC CBH HCB (intermediate computation bc eyes are x-ing)
// NBCCNBBBCBHCB (real result)
// NB BC CC CN NB BB BB BC CB BH HC CB (new pair counts: 12)

// So, each pair transforms into two (arbitrary, must look up) different pairs on the next step, but the pairs aren't the polymer; they have some redundancy. Where's that redundancy at?

// Oh! every letter except the first and last is included twice in the pair sets! So I think it's
// - Remember first and last letter.
// - Use multiplication to find letter counts from the pair sets.
// - Divide each bucket in half.
// - ADD first and last letter back to their respective buckets.

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
