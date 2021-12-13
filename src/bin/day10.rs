use advent21::*;

// the one with the bytecode interpreter and the syntax errors
fn main() {
    let inputs = load_inputs("day10").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// huck any corrupted lines, close any unclosed delimiters in correct order for
// each incomplete line and calculate the completion score, then pick the
// _middle_ completion score (apparently there will definitely be an odd
// number).
fn part_two(inputs: &str) -> usize {
    let mut valid_scores: Vec<usize> = Vec::new();
    for line in inputs.lines() {
        if let Some(completion) = build_completion_string(line) {
            valid_scores.push(score_completion_string(&completion));
        }
    }
    valid_scores.sort();
    println!("Got some completion scores: \n{:?}", &valid_scores);
    if valid_scores.len() % 2 != 1 { panic!("just checking!"); }
    // integer division floors the result, but bc collections are 0-indexed that
    // ends up being the right index to use.
    let median_score = valid_scores[ valid_scores.len() / 2 ];
    println!("Median score is {}", median_score);
    median_score
}

// find first illegal delimiter on each line (if present), give it a score, and
// total the scores.
// All right, I have a sneaking suspicion that ANY optimization on this one is premature.
// Let's just use a stack and holler when we pop off something bad.
fn part_one(inputs: &str) -> usize {
    let total_score = inputs.lines().map(score_corrupted_line).fold(
        0usize,
        |total, maybe_score| {
            let score = match maybe_score {
                None => 0,
                Some(n) => n,
            };
            total + score
        }
    );
    println!("total score: {}", total_score);
    total_score
}

fn score_completion_string(completion: &str) -> usize {
    completion.chars().fold(0usize, |score, c| {
        score * 5 + pay_me(c)
    })
}

// None means it was a corrupted line.
fn build_completion_string(line: &str) -> Option<String> {
    let mut stack: Vec<char> = Vec::new();
    for ch in line.chars() {
        if is_opener(ch) {
            stack.push(ch);
        } else if is_closer(ch) {
            let opener = stack.pop();
            match opener {
                None => { return None; },
                Some(opener) => {
                    if !delimiters_are_matching(opener, ch) {
                        return None;
                    }
                }
            }
        } else {
            panic!("shouldn't happen.");
        }
    }
    // At this point, we have some remaining ... stuff. Oh, and we don't need to
    // handle the "nothing's fucked" case, if we're feeling lazy, bc the puzzle
    // setup guarantees we're good to go.

    // Oh! Wait! It should ONLY be opening brackets left, because we should have
    // popped matching openers off every time we hit a closer. So the remaining
    // stuff actually doesn't resemble the string we initially got -- it's got
    // any "resolved" pairs snipped out. So... g2g~!
    let mut completions = String::new();
    while let Some(ch) = stack.pop() {
        completions.push(closer_for(ch));
    }
    Some(completions)
}

fn score_corrupted_line(line: &str) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    for ch in line.chars() {
        if is_opener(ch) {
            stack.push(ch);
        } else if is_closer(ch) {
            let opener = stack.pop();
            match opener {
                None => {
                    // A closer with no opener.
                    return Some(punish_me_daddy(ch));
                },
                Some(opener) => {
                    if !delimiters_are_matching(opener, ch) {
                        // A closer with the wrong opener.
                        return Some(punish_me_daddy(ch))
                    }
                },
            }
        } else {
            panic!("oops, unexpected input");
        }
        // OK, this char was fine! keep going.
    }

    None
}

fn is_opener(ch: char) -> bool {
    match ch {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}
fn is_closer(ch: char) -> bool {
    match ch {
        ')' | ']' | '}' | '>' => true,
        _ => false,
    }
}

const BAD_PAREN: usize = 3;
const BAD_SQUARE: usize = 57;
const BAD_CURLY: usize = 1197;
const BAD_ANGLE: usize = 25137;

fn delimiters_are_matching(op: char, cl: char) -> bool {
    (op == '(' && cl == ')')
    || (op == '[' && cl == ']')
    || (op == '{' && cl == '}')
    || (op == '<' && cl == '>')
}

fn pay_me(work: char) -> usize {
    match work {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("come ON,"),
    }
}

fn closer_for(opener: char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("oops, nope"),
    }
}

fn punish_me_daddy(badness: char) -> usize {
    match badness {
        ')' => BAD_PAREN,
        ']' => BAD_SQUARE,
        '}' => BAD_CURLY,
        '>' => BAD_ANGLE,
        _ => panic!("Oops, wyd"),
    }
}

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
        let answer = 288957;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn score_some_completion_strings() {
        assert_eq!(score_completion_string("])}>"), 294);
        assert_eq!(score_completion_string("}}]])})]"), 288957);
        assert_eq!(score_completion_string(")}>]})"), 5566);
        assert_eq!(score_completion_string("}}>}>))))"), 1480781);
    }
}
