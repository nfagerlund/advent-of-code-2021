use std::collections::{HashMap, HashSet};
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
// No wait, I have an easier way to get d (hm). Find the 4-length signal, and find the segment that isn't claimed by the verticals.
fn part_two(inputs: &str) -> usize {
    let mut the_stuff = parse_inputs_weirdly(inputs);
    println!("the stuff is here, showing first one:\n{:#?}", &the_stuff[0]);
    let digit_recognizer = build_digit_recognizer();

    // let (temp_sigs, temp_digits) = the_stuff.swap_remove(0);
    // let temp_disp = decode_display(temp_sigs, temp_digits);
    // println!("Temp display, only partially filled: \n{:#?}", &temp_disp);
    let temp_sigs = &the_stuff[0].0;
    let decoded = decode_segments(temp_sigs);
    println!("OK, here's what I got for that decoded display: \n{:#?}", &decoded);
    let readout = &the_stuff[0].1;
    println!("Printing the display contents:");
    for digit in readout {
        let real_digit = translate_digit(digit, &decoded, &digit_recognizer);
        println!("{}", real_digit);
    }
    0
}

// Change of plan: I think decoding the segments is technically more computation
// work than needed, but I'm coming to believe it'll mean less *coding* work.
// Also, do it immutably, bc fuck what was happening earlier.
// The comments describing this logic are up around part_two().
fn decode_segments(signals: &Vec<&str>) -> SegmentTranslator {
    let mut segments: HashMap<char, Segment> = HashMap::new();
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for signal in signals {
        for c in signal.chars() {
            let freq = frequencies.entry(c).or_insert(0);
            *freq += 1;
        }
    }
    // Easy buckets first
    for (c, freq) in &frequencies {
        match *freq {
            9 => { segments.insert(*c, Segment::Vbr); },
            4 => { segments.insert(*c, Segment::Vbl); },
            6 => { segments.insert(*c, Segment::Vul); },
            _ => {},
        }
    }
    // Then that last vertical
    for signal in signals {
        if signal.len() == 2 {
            // it's the "1"
            for c in signal.chars() {
                if !segments.contains_key(&c) {
                    // then it's not Vbr (which is already stored), and must be Vur
                    segments.insert(c, Segment::Vur);
                }
            }
        }
    }
    // Then the top
    for (c, freq) in &frequencies {
        match *freq {
            8 => {
                if !segments.contains_key(c) {
                    // then it's not Vur and must be Ht
                    segments.insert(*c, Segment::Ht);
                }
            },
            _ => {},
        }
    }
    // Then the middle
    for signal in signals {
        if signal.len() == 4 {
            for c in signal.chars() {
                if !segments.contains_key(&c) {
                    segments.insert(c, Segment::Hm);
                }
            }
        }
    }
    // Then the bottom is all that's left
    for (c, _) in frequencies {
        if !segments.contains_key(&c) {
            segments.insert(c, Segment::Hb);
        }
    }

    segments
}

// Ok, onward!
fn translate_digit(digit_str: &str, translator: &SegmentTranslator, digit_recognizer: &DigitRecognizer) -> usize {
    let lit_segments: HashSet<Segment> = digit_str.chars()
        .map(|c| { translator[&c] }).collect();
    let (digit, _) = digit_recognizer.iter().find(|(_, segments)| {
        (*segments).eq(&lit_segments)
    }).unwrap();
    *digit
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Segment {
    Ht,
    Hm,
    Hb,
    Vul,
    Vur,
    Vbl,
    Vbr,
}

// Okay, I don't like what I'm about to do here, but I spent long enough trying
// to think of something nicer.
// This is the part that's standing in for human vision, basically.
type DigitRecognizer = HashMap<usize, HashSet<Segment>>;
fn build_digit_recognizer() -> DigitRecognizer {
    let mut segment_sets = HashMap::new();
    for i in 0usize..=9 {
        let mut segments = HashSet::new();
        match i {
            0 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbl);
                segments.insert(Segment::Vbr);
            },
            1 => {
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbr);
            },
            2 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbl);
            },
            3 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbr);
            },
            4 => {
                segments.insert(Segment::Hm);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbr);
            },
            5 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vbr);
            },
            6 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vbl);
                segments.insert(Segment::Vbr);
            },
            7 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbr);
            },
            8 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vbr);
                segments.insert(Segment::Vbl);
            },
            9 => {
                segments.insert(Segment::Ht);
                segments.insert(Segment::Hm);
                segments.insert(Segment::Hb);
                segments.insert(Segment::Vul);
                segments.insert(Segment::Vur);
                segments.insert(Segment::Vbr);
            },
            _ => { panic!("hey wtf") },
        }
        segment_sets.insert(i, segments);
    }
    segment_sets
}

type SegmentTranslator = HashMap<char, Segment>;

fn vec_of_digits_to_decimal(digits: &Vec<usize>) -> usize {
    digits.iter().rev().enumerate().fold(
        0,
        |sum, (i, &val)| {
            sum + (val * 10usize.pow(i as u32))
        }
    )
}

// (signals, digits)
fn parse_inputs_weirdly<'a>(inputs: &'a str) -> Vec<(Vec<&'a str>, Vec<&'a str>)> {
    let tuples = parse_inputs_naively(inputs);
    let vec_of_displays: Vec<(Vec<&str>, Vec<&str>)> = tuples.iter()
        .map(|(ten_signals, four_digits)| {
            let signals: Vec<&str> = ten_signals.split(' ').collect();
            let digits: Vec<&str> = four_digits.split(' ').collect();
            (signals, digits)
        }).collect();
    vec_of_displays
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
