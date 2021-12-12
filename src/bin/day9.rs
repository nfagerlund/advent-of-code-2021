use advent21::*;

// The one with the smoke seeking low points.
fn main() {
    let inputs = load_inputs("day9").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) {}

fn part_one(inputs: &str) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn example_part_one() {
        let answer = 15;
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
