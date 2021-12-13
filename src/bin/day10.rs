use advent21::*;

// the one with the bytecode interpreter and the syntax errors
fn main() {
    let inputs = load_inputs("day10").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// find first illegal delimiter on each line (if present), give it a score, and
// total the scores.
fn part_one(inputs: &str) -> usize {

    0
}

const BAD_PAREN: usize = 3;
const BAD_SQUARE: usize = 57;
const BAD_CURLY: usize = 1197;
const BAD_ANGLE: usize = 25137;

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
"#;

    #[test]
    fn example_part_one() {
        let answer = 26397;
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
