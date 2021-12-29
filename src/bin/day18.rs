use advent21::*;
use std::fmt;

// the one with the snailfish numbers.
fn main() {
    let inputs = load_inputs("day18").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> u32 {

    0
}

#[derive(Debug)]
enum Sn {
    Regular(u32),
    Pair(Vec<Sn>),
}

impl Sn {
    fn textualize(&self) -> String {
        match self {
            Sn::Regular(num) => {
                format!("{}", num)
            },
            Sn::Pair(list) => {
                format!("[{},{}]", list[0].textualize(), list[1].textualize())
            },
        }
    }
}

impl fmt::Display for Sn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.textualize())
    }
}

fn parse_line(line: &str) -> Sn {
    let mut stack: Vec<Sn> = Vec::new();
    for ch in line.chars() {
        if ch == '[' {
            stack.push(Sn::Pair(vec![]));
        } else if ch == ',' {
            // do nothing
        } else if let Some(digit) = ch.to_digit(10) {
            if let Sn::Pair(ref mut current) = stack.last_mut().unwrap() {
                current.push(Sn::Regular(digit));
            }
        } else if ch == ']' && stack.len() > 1 {
            let done = stack.pop().unwrap();
            if let Sn::Pair(ref mut current) = stack.last_mut().unwrap() {
                current.push(done);
            }
        }
    }
    if stack.len() != 1 {
        panic!("oops, messed up");
    }
    return stack.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn example_part_one() {
        let answer = 4140;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }

    // This is more a manual test than a "test" test.
    #[test]
    fn parse_line_maybe() {
        let line = EXAMPLE.lines().next().unwrap();
        let result = parse_line(line);
        dbg!(&result);
        println!("{}", &result);
        let text = format!("{}", &result);
        assert_eq!(&text[..], line);
    }
}
