use advent21::*;

// The one with decoding some kind of hellish binary wire format
fn main() {
    let inputs = load_inputs("day16").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

fn part_one(inputs: &str) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "";

    #[test]
    fn example_part_one() {
        // let answer = ();
        // let result = part_one(EXAMPLE);
        // assert_eq!(result, answer);
        // This one's a bit different, so it has a bunch of different examples:
        assert_eq!(16, part_one("8A004A801A8002F478"));
        assert_eq!(12, part_one("620080001611562C8802118E34"));
        assert_eq!(23, part_one("C0015000016115A2E0802F182340"));
        assert_eq!(31, part_one("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
