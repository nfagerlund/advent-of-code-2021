use advent21::*;

fn main() {
    let inputs = load_inputs("day3").unwrap();
    part_two(inputs);
}

fn part_two(inputs: String) -> i32 {
    let mut multiplied = 0;

    multiplied
}

fn part_one(inputs: String) -> i32 {
    let mut multiplied = 0;
    // Right. So. This one is goofy.
    // It looks like we can get away with only tracking a running sum for each
    // binary digit. And the epsilon rate is just the gamma rate with every bit
    // flipped.
    // Uhhhhh we have 1000 numbers to work with, and that is an even number,
    // sooooooo... there's a chance of a tie, and the problem statement doesn't
    // say how to break a tie.
    // I say we panic on tie.
    // SO: a running sum integer for each digit. We ++ for 1, and -- for 0.
    // At end, if sum > 0 gamma gets 1, if sum < 0 gamma gets 0, if sum == 0 panic.
    // Then we need to flip bits for epsilon.
    // Then we need to parse int from str of radix 2.
    // Then multiply.

    // K let's do this. char_indices is the inner iterator we want, and we can
    // collect our stuff in a vec.

    let digit_accumulators = digit_frequencies(&inputs);
    // Now... Maybe let's use bit shifting, if I can do so without wrecking self.
    let mut gamma = 0;
    let mut epsilon = 0;
    for place in digit_accumulators {
        gamma <<= 1;
        epsilon <<= 1;
        match place {
            0 => panic!("a digit frequency count tied, and I don't know how to break it"),
            (i32::MIN..=-1) => {
                // negative: gamma gets 0, epsilon gets 1
                epsilon += 1;
            },
            (1..=i32::MAX) => {
                // positive: gamma gets 1, epsilon gets 0
                gamma += 1;
            },
        }
    }
    // ok??
    multiplied = gamma * epsilon;
    println!("Gamma: {0:b} ({0})", gamma);
    println!("Epsilon: {0:b} ({0})", epsilon);
    println!("Multiplied: {}", multiplied);

    multiplied
}

fn digit_frequencies(inputs: &str) -> Vec<i32> {
    // I'm going to initialize the whole thing with zeroes first, just to make
    // the subsequent logic cleaner. Assume each number in input is same length.
    let input_width = inputs.lines().next().unwrap().len();
    let mut digit_accumulators: Vec<i32> = vec![0; input_width];

    for line in inputs.lines() {
        for (index, digit) in line.char_indices() {
            match digit {
                '0' => digit_accumulators[index] -= 1,
                '1' => digit_accumulators[index] += 1,
                _ => panic!("unexpected digit in input: {}", digit),
            }
        }
    }
    // k. At this point, we should have what we need in terms of counts...
    println!("{:?}", &digit_accumulators);
    // OK! At least there's no ties!
    digit_accumulators
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn example_part_one() {
        let multiplied: i32 = part_one(EXAMPLE.to_string());
        assert_eq!(multiplied, 198);
    }

    #[test]
    fn example_part_two() {
        let multiplied: i32 = part_two(EXAMPLE.to_string());
        assert_eq!(multiplied, 230);
    }
}
