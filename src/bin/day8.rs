use std::collections::HashMap;
use advent21::*;

// the case of the scrambled seven-segment displays

// Reminder to self:
// Digit -> # of segments
// 0 -> 6
// 1 -> 2
// 2 -> 5
// 3 -> 5
// 4 -> 4
// 5 -> 5
// 6 -> 6
// 7 -> 3
// 8 -> 7
// 9 -> 6

fn main() {
    let inputs = load_inputs("day8").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// All RIGHT! Now I need to ACTUALLY solve a puzzle!
// Translate every four-digit number, and sum them up.
// Right, first off, or I guess last off, the sequences are randomly ordered
// such that fcadb == fbcad. So, gotta sort/normalize them in between the
// decoding and translating stages.

// OK. There's three 6s, and three 5s.
// sixes: 0, 6, 9
// fives: 2, 5, 3
// I guess I'm looking for segments in common! Let's consider globally first.
// The three horizontal lines -- present in 2, 3, 5, 6, 9. Middle one is absent in 0.
    // 7 has the top line, plus the vertical lines from 1.
// The four vertical lines: all present in 0. 6 and 9 each have three. 2, 5, 3 each have two.
    // 2 and 5 have non-overlapping sets.
    // 3 has the same ones that 1 has!
    // 9 has the same ones that 4 has!
// Ok, well this is interesting... maybe a frequency count is a good first move, tho. Let's use the segment names from the example:
//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg
// Number of digits each segment appears in:
// horizontal lines:
// a: 8, d: 7, g: 7
// vertical lines:
// b: 6, e: 4, c: 8, f: 9
// Oh wow! So, you can figure out f (vbr), e (vbl), and b (vul) from freqs alone! That's 3 out of 4 verticals.
// For the final vertical, c (vur, x8): it and f are the only segments present in 1. So that's all them.
// There's only one other x8, so that's a (ht).
// d (hm) is the only horizontal segment that's absent from a six-segment digit. and g (hb) is whatever's left.
fn part_two(inputs: &str) -> usize {
    let mut the_stuff = parse_inputs_weirdly(inputs);
    println!("the stuff is here\n{:#?}", &the_stuff);
    let (temp_sigs, temp_digits) = the_stuff.swap_remove(0);
    let temp_disp = decode_display(temp_sigs, temp_digits);
    println!("Temp display, only partially filled: \n{:#?}", &temp_disp);
    0
}

fn decode_display(mut signals: Vec<String>, mut readout: Vec<String>) -> SevenSegmentDisplay {
    // ok, I'm just gonna do this one real dumb-like
    let mut readout_arr = [String::new(), String::new(), String::new(), String::new()];
        // WOW WHAT THE FUCK?? ^^
    for (i, s) in readout.drain(0..4).enumerate() {
        readout_arr[i] = s;
    }
    // at this point readout is consumed.
    let mut display = SevenSegmentDisplay {
        readout: readout_arr,
        digit_signals: HashMap::new(),
    };

    let (one, _) = signals.iter().enumerate().find(|(index, val)| {
        val.len() == 2
    }).unwrap();
    let one_s = signals.remove(one);
    display.digit_signals.insert(one_s, 1);
    // Wow, hated all of that ^^ I'm definitely fucking something up I think.

    display
}

// How many times do the digits 1, 4, 7, or 8 appear?
fn part_one(inputs: &str) -> usize {
    let mut easy_buckets: usize = 0;
    let badly_parsed_inputs = parse_inputs_naively(inputs);
    for display in badly_parsed_inputs {
        let four_digits = display.1;
        for digit in four_digits.split(' ') {
            match digit.len() { // just using ascii length, because.
                2 | 4 | 3 | 7 => easy_buckets += 1,
                _ => (),
            }
        }
    }
    println!("# of appearances of 1, 4, 7, or 8: {}", easy_buckets);
    easy_buckets
}

// uhhhhhh
// fuck it. (ten_signals, four_digits).
fn parse_inputs_naively(inputs: &str) -> Vec<(&str, &str)> {
    inputs.lines().map(|line| line.split_once(" | ").unwrap()).collect()
}

enum Segment {
    Ht,
    Hm,
    Hb,
    Vul,
    Vur,
    Vbl,
    Vbr,
    Unknown,
}

#[derive(Debug)]
struct SevenSegmentDisplay {
    digit_signals: HashMap<String, usize>,
    readout: [String; 4],
}

impl SevenSegmentDisplay {
    fn to_number(&self) -> usize {
        self.digit_signals[&self.readout[0]] * 1000
        +
        self.digit_signals[&self.readout[1]] * 100
        +
        self.digit_signals[&self.readout[2]] * 10
        +
        self.digit_signals[&self.readout[3]]
    }
}

// (signals, digits)
fn parse_inputs_weirdly(inputs: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let tuples = parse_inputs_naively(inputs);
    let vecs_of_sorted_strings: Vec<(Vec<String>, Vec<String>)> = tuples.iter()
        .map(|(ten_signals, four_digits)| {
            let signals: Vec<String> = ten_signals.split(' ')
                .map(sort_to_string).collect();
            let digits: Vec<String> = four_digits.split(' ')
                .map(sort_to_string).collect();
            (signals, digits)
        }).collect();
    vecs_of_sorted_strings
}

fn sort_to_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let mut result = String::new();
    for chr in chars {
        result.push(chr);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn example_part_one() {
        let answer = 26;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 61229;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
