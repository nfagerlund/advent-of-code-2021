use advent21::*;
use std::cell::RefCell;

fn main() {
    let inputs = load_inputs("day4").unwrap();
    part_one(&inputs);
    part_two(&inputs);
}

// Determine which board will win last, return its score
fn part_two(inputs: &str) -> i32 {
    let parsed_inputs = parse_inputs(&inputs);
    let called_numbers = parsed_inputs.0;
    let mut boards = parsed_inputs.1;
    println!("Using {} called numbers to check {} boards", called_numbers.len(), boards.len());
    // All the same so far. The key difference this time is,
    // - We don't stop when we hit a winning board.
    // - We keep track (mmm, or just calculate) of how many boards have already won, and make our call based on that.
    // Hmm. I think in order to keep track efficiently, we'll need a just_won function to supplant .winning() in this version.
    let mut active_boards_count = boards.len();
    for num in called_numbers {
        println!("calling {}", num);
        for board in boards.iter_mut() {
            board.mark(num);
            if board.just_won() {
                let score = board.score(num);
                println!("Found a winning board! Total score: {}", score);
                active_boards_count -= 1;
                println!("{} boards remaining.", active_boards_count);
                // and HERE's the money:
                if active_boards_count == 0 {
                    println!("And! That was the final winner!!");
                    return score;
                }
            }
        }
    }

    panic!("??? wtf? No one won last.")
}

// Determine which board will win first, return its score
fn part_one(inputs: &str) -> i32 {
    let parsed_inputs = parse_inputs(&inputs);
    let called_numbers = parsed_inputs.0;
    let mut boards = parsed_inputs.1;
    println!("Using {} called numbers to check {} boards", called_numbers.len(), boards.len());
    for num in called_numbers {
        println!("calling {}", num);
        for board in boards.iter_mut() {
            board.mark(num);
            if board.winning() {
                println!("Found a winning board!\n{:#?}", board);
                let score = board.score(num);
                println!("Total score: {}", score);
                return score
            }
        }
    }

    panic!("everybody loses!");
}

fn parse_inputs(inputs: &str) -> (Vec<i32>, Vec<Board>) {
    let mut blocks_iter = inputs.split("\n\n");
    let called_numbers: Vec<i32> = blocks_iter.next().unwrap().split(',')
        .map(|digit| { i32::from_str_radix(digit, 10).unwrap() }).collect();
    let boards: Vec<Board> = blocks_iter
        .map(|grid| {
            let squares = parse_5x5grid_to_squares(grid);
            Board::new(squares)
        })
        .collect();
    (called_numbers, boards)
}

fn _parse_5x5grid_to_vec(grid: &str) -> Vec<i32> {
    grid.split_whitespace()
        .map(|num_str| {
            i32::from_str_radix(num_str, 10).unwrap()
        })
        .collect()
}

fn parse_5x5grid_to_squares(grid: &str) -> Vec<Square> {
    grid.split_whitespace()
        .map(|num_str| {
            let id = i32::from_str_radix(num_str, 10).unwrap();
            Square::new(id)
        })
        .collect()
}

#[derive(Debug)]
pub struct Board {
    height: usize,
    width: usize,
    squares: Vec<Square>,
    won: RefCell<bool>,
}

impl Board {
    // This *moves* the provided vec of squares in, be warned!
    fn new(squares: Vec<Square>) -> Board {
        // hardcoding dimensions, meh
        let height: usize = 5;
        let width: usize = 5;
        // this should be a Result, but sshhhhh
        if squares.len() != height * width {
            panic!("wyd");
        }
        Board { height, width, squares, won: RefCell::new(false) }
    }

    fn mark(&mut self, num: i32) {
        // If this board just happens to be done, it's kind of consumed and we
        // don't need to do this extra work. But don't bother computing
        // .winning() early, bc it doesn't really matter if we do *one* tick of
        // unnecessary work, it's just a chance to be lazy.
        if self.cached_won() {
            return;
        }
        // OK, so,
        for square in self.squares.iter_mut().filter(|s| s.id == num ) {
            square.mark();
        }
    }

    fn cached_won(&self) -> bool {
        *(self.won.borrow())
    }

    fn cache_win(&self) {
        *(self.won.borrow_mut()) = true;
    }

    fn winning(&self) -> bool {
        if self.cached_won() {
            true
        } else {
            self.just_won()
        }
    }

    fn just_won(&self) -> bool {
        // If we learned that this board was done on a *previous* iteration, we
        // did NOT just win!
        if self.cached_won() {
            return false;
        }
        // OK, let's calm down a bit. Start with just rows, bc that will let us
        // find the winner for the example.
        for row in self.squares.chunks(self.width) {
            if line_wins(row.iter()) {
                println!("Found a winning row! {:#?}", row);
                self.cache_win();
                return true;
            }
        }
        // And THEN see if we can do columns:
        // Map<Range<usize>, |usize| -> Vec<&Square>>
        let columns_iter = (0..self.width).map(|col| {
            self.squares.iter().enumerate().filter(move |(index, _square)| {
                *index % self.width == col
            }).map(|(_, square)| square)
        });
        for column in columns_iter {
            let column: Vec<&Square> = column.collect();
            let sighhhhhh = column.clone();
            if line_wins(column.into_iter()) {
                println!("Found a winning column! {:#?}", sighhhhhh);
                self.cache_win();
                return true;
            }
        }

        false
    }

    // Not checking whether it's valid to HAVE a score -- so only call this if
    // you already know it won.
    fn score(&self, multiplier: i32) -> i32 {
        // total the remaining UN-marked squares:
        let sum = self.squares.iter().fold(0, |total, square| {
            if square.marked() {
                total
            } else {
                total + square.id
            }
        });
        // multiply by the final called number:
        sum * multiplier
    }
}

// just checking that it has NO unmarked squares
fn line_wins<'a, T: Iterator<Item = &'a Square>>(line: T) -> bool {
    line.filter(|sq| { !sq.marked() }).count() == 0
}

#[derive(Debug)]
pub struct Square {
    pub id: i32,
    marked: bool,
}

impl Square {
    fn new(id: i32) -> Square {
        Square {
            id,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn marked(&self) -> bool {
        self.marked
    }
}


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
        let answer = 1924;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
