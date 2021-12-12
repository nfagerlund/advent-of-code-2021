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

fn part_two(_inputs: &str) {}

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
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
