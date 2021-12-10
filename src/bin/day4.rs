use advent21::*;
use std::error::Error;

fn main() {
    let inputs = load_inputs("daySOMETHING").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

fn part_two(inputs: &str) {}

fn part_one(inputs: &str) -> i32 {
    println!("OKAY, SO:\n{:#?}", parse_inputs(&inputs));
    42 // TODO
}

fn parse_inputs(inputs: &str) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut blocks_iter = inputs.split("\n\n");
    let called_numbers: Vec<i32> = blocks_iter.next().unwrap().split(',')
        .map(|digit| { i32::from_str_radix(digit, 10).unwrap() }).collect();
    let boards: Vec<Vec<i32>> = blocks_iter
        .map(|grid| { parse_5x5grid_to_vec(grid) })
        .collect();
    (called_numbers, boards)
}

fn parse_5x5grid_to_vec(grid: &str) -> Vec<i32> {
    grid.split_whitespace()
        .map(|num_str| {
            i32::from_str_radix(num_str, 10).unwrap()
        })
        .collect()
}

pub struct Board {
    height: usize,
    width: usize,
    squares: Vec<Square>,
}

pub struct Square {}

// Row or column
pub struct Line {}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn example_part_one() {
        let answer = 4512;
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
