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

fn packet_bits_iterator(hex: &str) -> impl Iterator<Item = char> + '_ {
    hex.chars().map(|ch| {
        let num = ch.to_digit(16).unwrap();
        let bits_string = format!("{:b}", num);
        let bits: Vec<char> = bits_string.chars().collect();
        bits
    }).flatten()
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

    #[test]
    fn reliably_iterate_bits() {
        let example_str = "A0016C880162017C3686B18A3D4780";
        let control_val: u128 = u128::from_str_radix(example_str, 16).unwrap();
        let control_bits_string = format!("{:b}", control_val);
        let mut test_bits_string = String::new();
        for ch in packet_bits_iterator(example_str) {
            test_bits_string.push(ch);
        }
        assert_eq!(control_bits_string, test_bits_string);
    }
}
