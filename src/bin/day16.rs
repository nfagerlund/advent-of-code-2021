use advent21::*;

// The one with decoding some kind of hellish binary wire format
fn main() {
    let inputs = load_inputs("day16").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(_inputs: &str) {}

// okay uhhhhhhh let's not prematurely build-out on this one. Right now we're
// just summing the version fields? So we need a stack to pop packets on and off
// of, probably. And an accumulator. And as little as possible beyond that.
fn part_one(inputs: &str) -> usize {
    let mut bit_stream = packet_bits_iterator(inputs);
    let parse_stack: Vec<Length> = Vec::new();
    let outer_ver = take_number(&mut bit_stream, 3);
    let outer_type = take_number(&mut bit_stream, 3);
    println!("outer packet\nver: {}\ntype: {}", outer_ver, outer_type);
    if outer_type != 4 {
        match bit_stream.next().unwrap() {
            '0' => {
                let length = take_number(&mut bit_stream, 15);
                println!("bit-length mode: inner packets take {} bits", length);
            },
            '1' => {
                let length = take_number(&mut bit_stream, 11);
                println!("packet count mode: {} inner packets", length);
            },
            _ => panic!("wyd"),
        };
    }

    0
}

// and here's our state machine step function.
// things where I know I'm being bad: I'm not doing anything to guard against
// malformed input of any kind. deal w/ it.
fn parse_bit_stream_step<T: Iterator<Item = char>>(
    state: ParseState, stack: &mut Vec<Packet>, bit_stream: &mut T
) -> ParseState {
    match state {
        ParseState::StartPacket => {
            let version = take_number(bit_stream, 3);
            let type_id = take_number(bit_stream, 3);
            let encoded_size = 6_usize;
            let contents = match type_id {
                4 => Contents::Literal(0),
                _ => Contents::Operator(Vec::new()),
            };
            let length = match type_id {
                4 => Length::Literal,
                _ => Length::Unknown,
            };
            let sum_of_child_versions = 0_usize;
            let packet = Packet {
                version,
                type_id,
                encoded_size,
                contents,
                length,
                sum_of_child_versions,
            };
            stack.push(packet);
            // and, return!
            match type_id {
                4 => ParseState::ParseValue,
                _ => ParseState::ParseLength,
            }
        },
        ParseState::ParseValue => {
            // lazy lazy
            let current = stack.last_mut().unwrap();
            // eating five bits no matter what:
            current.encoded_size += 5;
            let is_final_chunk = match take_number(bit_stream, 1) {
                0 => true,
                _ => false,
            };
            let chunk = take_number(bit_stream, 4);
            // should always be true: vv
            if let Contents::Literal(ref mut value) = current.contents {
                *value += chunk;
            }
            // and return!
            if is_final_chunk {
                ParseState::EndPacket
            } else {
                ParseState::ParseValue
            }
        },
        ParseState::ParseLength => {
            let current = stack.last_mut().unwrap();
            let length_type = take_number(bit_stream, 1);
            match length_type {
                0 => {
                    // bits mode, length is next 15 bits.
                    current.encoded_size += 1 + 15;
                    let length = take_number(bit_stream, 15);
                    current.length = Length::Bits(length);
                },
                1 => {
                    // count mode, length is next 11 bits
                    current.encoded_size += 1 + 11;
                    let length = take_number(bit_stream, 11);
                    current.length = Length::Count(length);
                },
                _ => panic!("wyd, thought this was bits"),
            }
            // and return! at this point, we just started an operator packet,
            // its entire content will be composed of other packets. So, we
            // unconditionally jump back to start packet.
            ParseState::StartPacket
        },
        ParseState::EndPacket => {},
        ParseState::Finished => {
            ParseState::Finished
        },
    }
}

enum ParseState {
    StartPacket, // get ver/type header, make & push packet skeleton, jump to ParseValue or ParseLength
    ParseValue, // continue til done, then jump to EndPacket
    ParseLength, // Determine length type and length, then jump to StartPacket (!)
    EndPacket, // the big one. If there's no parent, leave packet on stack and jump to Finished. If there is a parent, pop packet off stack, add it to children of parent, decrement parent length. If parent length is exhausted, jump to EndPacket again; otherwise jump to StartPacket.
    Finished, // do nothing forever.
}

enum Length {
    Unknown,
    Literal,
    Bits(usize),
    Count(usize),
}

enum Contents {
    Operator(Vec<Packet>),
    Literal(usize),
}

struct Packet {
    version: usize,
    type_id: usize,
    contents: Contents,
    length: Length, // gets decremented over time, we eventually forget the length.
    encoded_size: usize, // needed if parent's length is Bits. Remember to increment parent's encoded size at End.
    sum_of_child_versions: usize, // eh fuck it
}

/// This might return a vec with fewer than requested elements, if we flip past
/// the end of the iterator.
fn take_n<T: Iterator<Item = char>>(iter: &mut T, n: usize) -> Vec<char> {
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(item) = iter.next() {
            result.push(item);
        }
    }
    result
}

fn num_from_charbits(bits: &Vec<char>) -> usize {
    bits.iter().fold(0_usize, |accum, ch| {
        let new_bit = ch.to_digit(2).unwrap() as usize;
        (accum << 1) + new_bit
    })
}

fn take_number<T: Iterator<Item = char>>(iter: &mut T, n: usize) -> usize {
    let charbits = take_n(iter, n);
    num_from_charbits(&charbits)
}

fn packet_bits_iterator(hex: &str) -> impl Iterator<Item = char> + '_ {
    hex.chars().map(|ch| {
        let num = ch.to_digit(16).unwrap();
        let bits_string = format!("{:04b}", num);
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

    #[test]
    fn reliably_take_n() {
        let mut bits_iter = packet_bits_iterator("abc");
        // 1010 1011 1100
        assert_eq!(take_n(&mut bits_iter, 3), vec!['1', '0', '1']);
        assert_eq!(take_n(&mut bits_iter, 3), vec!['0', '1', '0']);
        assert_eq!(take_n(&mut bits_iter, 3), vec!['1', '1', '1']);
    }

    #[test]
    fn reliably_parse_charbits() {
        assert_eq!(num_from_charbits(&vec!['1', '0', '1']), 5);
    }
}
