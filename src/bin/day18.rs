use advent21::*;
use std::fmt;
use std::mem;

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

#[derive(Debug, PartialEq, Eq)]
enum Reduction {
    Splode(Option<u32>, Option<u32>),
    Split,
    Nope,
}

enum Side {
    L,
    R,
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

    // Mutate self in-place to perform a single reduction step. Return the
    // result, which the caller might need to act on. `level` is the level of
    // recursion we're acting at, since that's relevant for splode.
    fn reduce_step(&mut self, level: u32) ->  Reduction {
        match self {
            Sn::Regular(num) => {
                let num = *num;
                if num > 9 {
                    // make like a banana and perform nuclear fission
                    let left = num / 2;
                    let right = num / 2 + num % 2;
                    let replacement = Sn::Pair(vec![Sn::Regular(left), Sn::Regular(right)]);
                    let _ = mem::replace(self, replacement);
                    Reduction::Split
                } else {
                    // yay finally
                    Reduction::Nope
                }
            },
            Sn::Pair(pair) => {
                // OK, here comes the fun one. 😤
                if level > 4 {
                    panic!("Theoretically, the rules say this should never happen.");
                } else if level == 4 {
                    // splode!
                    // contents should 100% be regulars at this point; extract them.
                    let left = match pair[0] {
                        Sn::Regular(num) => num,
                        _ => panic!("wyd"),
                    };
                    let right = match pair[1] {
                        Sn::Regular(num) => num,
                        _ => panic!("wyd"),
                    };
                    // replace self with 0
                    let _ = mem::replace(self, Sn::Regular(0));
                    // return splode
                    Reduction::Splode(Some(left), Some(right))
                } else {
                    // recurse! respond to first result found, and return similar.
                    let mut result = pair[0].reduce_step(level + 1);
                    let mut result_side = Side::L;
                    if result == Reduction::Nope {
                        // only run right reduction if left whiffed.
                        result = pair[1].reduce_step(level + 1);
                        result_side = Side::R;
                    }
                    match result {
                        // The first two are easy: just propagate so we know to bail.
                        Reduction::Nope => Reduction::Nope,
                        Reduction::Split => Reduction::Split,
                        Reduction::Splode(mut sp_left, mut sp_right) => {
                            // ok, now how did this go...?
                            match result_side {
                                Side::L => {
                                    // If left is sploding, see if we need to dispose of the right component.
                                    if let Some(num) = sp_right.take() {
                                        pair[1].absorb_splode(num, Side::L);
                                    }
                                },
                                Side::R => {
                                    // If right is sploding, see if we need to dispose of left.
                                    if let Some(num) = sp_left.take() {
                                        pair[0].absorb_splode(num, Side::R);
                                    }
                                },
                            }
                            // then propagate whatever's left of the splode components.
                            Reduction::Splode(sp_left, sp_right)
                        }
                    }
                }
            },
        }
    }

    // more recursive funtimes. This doesn't return anything bc we don't really
    // need to know if it succeeds or not; a splode component that doesn't find
    // a home just drifts off into the void.
    fn absorb_splode(&mut self, val: u32, from: Side) {
        match self {
            Sn::Regular(num) => {
                // sweet, done.
                let num = *num;
                let _ = mem::replace(self, Sn::Regular(num + val));
            },
            Sn::Pair(pair) => {
                // ok...
                let index: usize = match from {
                    Side::L => 0,
                    Side::R => 1,
                };
                pair[index].absorb_splode(val, from);
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

    #[test]
    fn replace_self_maybe() {
        // I'm just really curious
        let mut simple = Sn::Regular(4);
        let paired = Sn::Pair(vec![Sn::Regular(8), Sn::Regular(2)]);
        let simpref = &mut simple;
        match simpref {
            Sn::Regular(_) => {
                let num = std::mem::replace(simpref, paired);
                println!("num: {}", num);
            },
            _ => {},
        }
        println!("{}", &simple);
        // wow!!! I was so sure that wouldn't work! so glad I read that linked lists book.
    }

    #[test]
    fn try_split() {
        let mut sn = Sn::Regular(13);
        println!("{}", &sn);
        let reduction = sn.reduce_step(0);
        println!("{:?}\n{}", reduction, &sn);
    }

    #[test]
    fn try_splode() {
        let mut sn = Sn::Pair(vec![Sn::Regular(4), Sn::Regular(9)]);
        println!("{}", &sn);
        let reduction = sn.reduce_step(4);
        println!("{:?}\n{}", reduction, &sn);

        let bigger = parse_line(EXAMPLE.lines().next().unwrap());
        let smaller = Sn::Pair(vec![Sn::Regular(4), Sn::Regular(9)]);
        let mut combined = Sn::Pair(vec![bigger, smaller]);
        println!("{}", &combined);
        let reduction = combined.reduce_step(0);
        println!("{:?}\n{}", reduction, &combined);
        let reduction = combined.reduce_step(0);
        println!("{:?}\n{}", reduction, &combined);
        let reduction = combined.reduce_step(0);
        println!("{:?}\n{}", reduction, &combined);
    }

    #[test]
    fn try_absorb() {
        let mut sn = Sn::Pair(vec![
            Sn::Regular(4),
            Sn::Pair(vec![
                Sn::Regular(9),
                Sn::Regular(6),
            ]),
        ]);
        println!("{}", &sn);
        sn.absorb_splode(4, Side::R);
        println!("{}", &sn);
        sn.absorb_splode(4, Side::L);
        println!("{}", &sn);
    }
}
