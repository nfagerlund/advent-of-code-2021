use advent21::*;
use std::fmt;
use std::mem;

// the one with the snailfish numbers.
fn main() {
    let inputs = load_inputs("day18").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// what's the biggest magnitude you can get by adding only two of the numbers?
fn part_two(inputs: &str) -> u32 {
    // combinatorial explosion, cool cool cool. should be fine?
    // vv NOT mut! must clone!
    let sns: Vec<Sn> = inputs.lines().map(|line| parse_line(line)).collect();
    let magnitudes: Vec<u32> = sns.iter().enumerate().map(|(which_left, left)| {
        let sub_magnitudes: Vec<u32> = sns.iter().enumerate().map(|(which_right, right)| {
            if which_left == which_right {
                0 // cheating :] won't be greatest magnitude
            } else {
                let lhs = left.clone();
                let rhs = right.clone();
                lhs.add(rhs).magnitude()
            }
        }).collect();
        sub_magnitudes
    }).flatten().collect();
    println!("All magnitudes: \n{:?}", &magnitudes);
    let max = magnitudes.iter().max().unwrap();
    println!("Biggest magnidude: {}", max);
    *max
}

// Sum whole list of snailfish numbers
fn part_one(inputs: &str) -> u32 {
    let final_sum = inputs.lines().map(parse_line).reduce(|accum, val| {
        accum.add(val)
    }).unwrap();
    println!("Final sum: {}", &final_sum);
    let magnitude = final_sum.magnitude();
    println!("Magnitude of final sum: {}", magnitude);
    magnitude
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

#[derive(Debug, Clone)]
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

    fn magnitude(&self) -> u32 {
        match self {
            Sn::Regular(num) => {
                *num
            },
            Sn::Pair(pair) => {
                pair[0].magnitude() * 3 + pair[1].magnitude() * 2
            },
        }
    }

    // modify in-place to reduce.
    fn reduce(&mut self) {
        while self.reduce_step() != Reduction::Nope {}
    }

    // Mutate self in-place to perform a single reduction step. Return the
    // result, which the caller might need to act on.
    fn reduce_step(&mut self) ->  Reduction {
        // OK, we're currently outside the recursion -- if you're calling this
        // method, you're treating this Sn as the root.
        let mut result = self.reduce_splode_step(0);
        if result == Reduction::Nope {
            result = self.reduce_split_step(0);
        }
        // ...I think that's it?
        result
    }

    // Recursively process ONLY PLODES. `level` is the level of
    // recursion we're acting at, since that's relevant for splode.
    fn reduce_splode_step(&mut self, level: u32) -> Reduction {
        match self {
            Sn::Regular(_) => {
                Reduction::Nope
            },
            Sn::Pair(pair) => {
                if level > 4 {
                    panic!("Theoretically, the rules say this should never happen.");
                } else if level == 4 {
                    // splode
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
                    // recurse, then process a descendent splode if necessary.
                    let mut result = pair[0].reduce_splode_step(level + 1);
                    let mut result_side = Side::L;
                    if result == Reduction::Nope {
                        // Only reduce right side if left was fully reduced already.
                        result = pair[1].reduce_splode_step(level + 1);
                        result_side = Side::R;
                    }
                    match result {
                        Reduction::Nope => Reduction::Nope, // propagate+bail
                        Reduction::Split => panic!("*pounds car hood* hey, I'm sploding here!!"),
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

    fn reduce_split_step(&mut self, level: u32) -> Reduction {
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
                // recurse, and just propagate the result. No further processing needed.
                let mut result = pair[0].reduce_split_step(level + 1);
                if result == Reduction::Nope {
                    result = pair[1].reduce_split_step(level + 1);
                }
                // don't care about which side it was this time.
                result
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

    fn add_unreduced(self, other: Sn) -> Sn {
        Sn::Pair(vec![self, other])
    }

    // Finally: Consume self and other to return a reduced sum.
    fn add(self, other: Sn) -> Sn {
        println!("  {}", &self);
        println!("+ {}", &other);
        let mut result = Sn::Pair(vec![self, other]);
        result.reduce();
        println!("= {}\n", &result);
        result
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
    use std::collections::HashMap;

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
    fn example_part_two() {
        let answer = 3993;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_one() {
        let answer = 4140;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn another_add_test() {
        let text = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
        part_one(text);
    }

    #[test]
    fn yet_another_add_test() {
        let answer = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]";
        let first = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        let second = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let result = parse_line(first).add(parse_line(second));
        let result_text = format!("{}", &result);
        assert_eq!(answer, result_text);
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
        let reduction = sn.reduce_step();
        println!("{:?}\n{}", reduction, &sn);
    }

    #[test]
    fn try_splode() {
        let mut sn = Sn::Pair(vec![Sn::Regular(4), Sn::Regular(9)]);
        println!("{}", &sn);
        let reduction = sn.reduce_splode_step(4);
        println!("{:?}\n{}", reduction, &sn);

        let bigger = parse_line(EXAMPLE.lines().next().unwrap());
        let smaller = Sn::Pair(vec![Sn::Regular(4), Sn::Regular(9)]);
        let mut combined = Sn::Pair(vec![bigger, smaller]);
        println!("{}", &combined);
        let reduction = combined.reduce_step();
        println!("{:?}\n{}", reduction, &combined);
        let reduction = combined.reduce_step();
        println!("{:?}\n{}", reduction, &combined);
        let reduction = combined.reduce_step();
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

    #[test]
    fn full_reduction_test() {
        let mut examples = HashMap::new();
        examples.insert("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        examples.insert(
            "[[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]],[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
        );
        examples.insert(
            "[[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );
        examples.insert(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
        for (unreduced, reduced) in examples {
            let mut working_copy = parse_line(unreduced);
            working_copy.reduce();
            let result = format!("{}", &working_copy);
            assert_eq!(&result[..], reduced);
        }
    }

    #[test]
    fn magnitude_test() {
        let mut examples = HashMap::new();
        examples.insert("[[9,1],[1,9]]", 129);
        examples.insert("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488);
        examples.insert("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        examples.insert("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        examples.insert("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        examples.insert("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        for (text, result) in examples {
            let working_copy = parse_line(text);
            assert_eq!(working_copy.magnitude(), result);
        }
    }
}
