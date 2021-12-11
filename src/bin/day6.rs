use advent21::*;

fn main() {
    let inputs = load_inputs("day6").unwrap();
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
    const EXAMPLE: &str = "3,4,3,1,2\n";

    #[test]
    fn example_part_one() {
        let answer = 5934;
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
