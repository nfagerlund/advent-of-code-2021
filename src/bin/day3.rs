use advent21::*;

fn main() {
    let inputs = load_inputs("day3").unwrap();
    part_one(inputs);
}

fn part_one(inputs: String) -> i32 {
    println!("hi.");
    let multiplied = 0;

    multiplied
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
}
