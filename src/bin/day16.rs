use advent21::*;
use std::cmp;

// The one with decoding some kind of hellish binary wire format
fn main() {
    let inputs = load_inputs("day16").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) -> usize {

    0
}

// okay uhhhhhhh let's not prematurely build-out on this one. Right now we're
// just summing the version fields? So we need a stack to pop packets on and off
// of, probably. And an accumulator. And as little as possible beyond that.
fn part_one(inputs: &str) -> usize {
    let mut bit_stream = packet_bits_iterator(inputs);
    let mut parse_stack: Vec<Packet> = Vec::new();
    let mut parse_state = ParseState::StartPacket;
    while parse_state != ParseState::Finished {
        dbg!(&parse_state);
        parse_state = parse_bit_stream_step(parse_state, &mut parse_stack, &mut bit_stream);
    }

    // that should be it?
    assert_eq!(parse_stack.len(), 1);
    let outer_packet = parse_stack.get(0).unwrap();
    let version_sum = outer_packet.sum_of_child_versions + outer_packet.version;
    println!("Sum of all packet versions: {}", version_sum);

    version_sum
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
        ParseState::EndPacket => {
            // First the easy exit:
            if stack.len() <= 1 {
                return ParseState::Finished;
            }
            // Ok so: we now know there's a parent, which is definitely an Operator.
            // grab the warp core:
            let current = stack.pop().unwrap();
            // Get to the vessel:
            let parent = stack.last_mut().unwrap();
            // Increment parent sums:
            parent.encoded_size += current.encoded_size;
            parent.sum_of_child_versions += current.version + current.sum_of_child_versions;
            // Hold onto child length for a sec:
            let current_encoded_size = current.encoded_size;
            // Absorb finished packet: (vv always true)
            if let Contents::Operator(ref mut children) = parent.contents {
                children.push(current); // <- move
            }
            // Decrement parent length... and return!
            match parent.length {
                Length::Bits(ref mut length) => {
                    *length -= current_encoded_size;
                    if *length == 0 {
                        // end parent!
                        ParseState::EndPacket
                    } else {
                        // next child!
                        ParseState::StartPacket
                    }
                },
                Length::Count(ref mut length) => {
                    *length -= 1;
                    if *length == 0 {
                        // end parent!
                        ParseState::EndPacket
                    } else {
                        // next child!
                        ParseState::StartPacket
                    }
                },
                _ => panic!("len deffo shourdn't be unknown or literal at this point.")
            }
        },
        ParseState::Finished => {
            ParseState::Finished
        },
    }
}

// Consumes a packet and returns its reduced form (which will always be of type 4, literal.)
fn reduce_finished_packet(mut packet: Packet) -> Packet {
    // As expected, packet versions have no effect on anything lmao.
    match packet.type_id {
        4 => {
            // no-op, return the packet undigested.
            packet
        },
        0 => {
            // sum
            if let Contents::Operator(ref children) = packet.contents {
                let sum = children.iter().fold(0_usize, |accum, child| {
                    if let Contents::Literal(value) = child.contents {
                        accum + value
                    } else {
                        panic!("unreduced child in type 0 packet");
                    }
                });
                literalize(&packet, sum)
            } else {
                panic!("Bad child list for type 0 packet");
            }
        },
        1 => {
            // product
            if let Contents::Operator(ref children) = packet.contents {
                let product = children.iter().fold(1_usize, |accum, child| {
                    if let Contents::Literal(value) = child.contents {
                        accum * value
                    } else {
                        panic!("unreduced child in type 1 packet");
                    }
                });
                literalize(&packet, product)
            } else {
                panic!("Bad contents for type 1 packet");
            }
        },
        2 => {
            // min
            if let Contents::Operator(ref children) = packet.contents {

            } else {
                panic!("Bad contents for type 2 packet");
            }
        },
        3 => {
            // max
            if let Contents::Operator(ref children) = packet.contents {} else {
                panic!("Bad contents for type 3 packet");
            }
        },
        5 => {
            // > (1|0)
            if let Contents::Operator(ref children) = packet.contents {} else {
                panic!("Bad contents for type 5 packet");
            }
        },
        6 => {
            // < (1|0)
            if let Contents::Operator(ref children) = packet.contents {} else {
                panic!("Bad contents for type 6 packet");
            }
        },
        7 => {
            // = (1|0)
            if let Contents::Operator(ref children) = packet.contents {} else {
                panic!("Bad contents for type 7 packet");
            }
        },
        _ => panic!("Totally unknown packet type"),
    }
}

// Craft a new literal value packet, but use metadata fields from a provided operator packet.
fn literalize(packet: &Packet, value: usize) -> Packet {
    // Wanted to use struct update syntax, but it doesn't seem to work with
    // refs; it always consumes the referred-to value.
    Packet {
        version: packet.version,
        type_id: 4,
        contents: Contents::Literal(value),
        length: Length::Literal,
        encoded_size: packet.encoded_size,
        sum_of_child_versions: packet.sum_of_child_versions,
    }
}

#[derive(PartialEq, Eq, Debug)]
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
        assert_eq!(3, part_two("C200B40A82"));
        assert_eq!(54, part_two("04005AC33890"));
        assert_eq!(7, part_two("880086C3E88112"));
        assert_eq!(9, part_two("CE00C43D881120"));
        assert_eq!(1, part_two("D8005AC2A8F0"));
        assert_eq!(0, part_two("F600BC2D8F"));
        assert_eq!(0, part_two("9C005AC2F8F0"));
        assert_eq!(1, part_two("9C0141080250320F1802104A08"));
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
