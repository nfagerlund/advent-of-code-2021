use std::fs;
use std::fmt;
use std::error::Error;

fn load_inputs(dataset: &str) -> std::io::Result<String> {
    let file = format!("./inputs/{}.txt", dataset);
    fs::read_to_string(file)
}

// Count how many times the depth *increases*. Doesn't matter how much it
// increases by, doesn't matter how many measurements there were.
// Input is numbers separated by newlines.
fn single_comparison() {
    let inputs = load_inputs("day1a").unwrap();
    let mut last_sounding: Option<i32> = None;
    let mut increases = 0;
    for sounding in inputs.lines()
        .map(|l| { i32::from_str_radix(l, 10).unwrap() })
    {
        if let Some(previous) = last_sounding {
            if sounding > previous {
                increases += 1;
            }
        }
        last_sounding = Some(sounding);
    }

    println!("Depth increased {} times", increases);
}

// OK, now count how many times depth increases in adjacent three-measurement
// sliding windows:
fn main() {
    let inputs = load_inputs("day1a").unwrap();
    let soundings_iter = inputs.lines()
        .map(|l| { i32::from_str_radix(l, 10).unwrap() });
    // okay, uhhhhh
    // - Start accumulating a three-element window.
    // - when it's complete, compare it to the previous full window and then set
    //      previous = Some(current).
    // - But!!! Need to be accumulating multiple windows at once!! See, like this:
    //      one, two, three
    //          two, three, four
    //              three, four, five
    //      See, "three" joined three separate windows before scrolling off.
    // - If I'm to do this without losing track of what I'm doing, I'll want to
    //      have a struct with clean update methods! And I'm gonna have to
    //      actually manage borrow checking bc I can't just assign a
    //      non-primitive to a var outside the loop like that.
    // - While I'm at it, maybe I could implement PartialEq or something for the comparisons.
}

// Some custom errors... There must be a faster way to do this, but 🤷🏽
#[derive(Debug)]
pub struct WindowFullError;
impl fmt::Display for WindowFullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "can't add measurements to a completed measurement window")
    }
}

#[derive(Debug)]
pub struct WindowIncompleteError;
impl fmt::Display for WindowIncompleteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "can't read sum of an incomplete measurement window")
    }
}

// May as well generalize this to accept arbitrary window size.
pub struct Window {
    sum: i32,
    measurements: usize,
    capacity: usize,
}

impl Window {
    pub fn new(capacity: usize) -> Window {
        Window {
            sum: 0,
            measurements: 0,
            capacity,
        }
    }

    pub fn complete(&self) -> bool {
        self.measurements >= self.capacity
    }

    pub fn add(&mut self, measurement: i32) -> Result<(), WindowFullError> {
        if self.complete() {
            return Err(WindowFullError);
        }
        self.sum += measurement;
        self.measurements += 1;
        Ok(())
    }

    pub fn sum(&self) -> Result<i32, WindowIncompleteError> {
        if self.complete() {
            return Ok(self.sum);
        }
        Err(WindowIncompleteError)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition_and_completion() {
        let mut my_win = Window::new(3);
        my_win.add(2).unwrap();
        my_win.add(4).unwrap();
        assert_eq!(my_win.complete(), false);
        my_win.add(6).unwrap();
        assert_eq!(my_win.complete(), true);
        if let Ok(_) = my_win.add(2) {
            panic!("yo, adding to a complete window should have errored!");
        }
    }

    #[test]
    fn summation() {
        let mut my_win = Window::new(3);
        my_win.add(2).unwrap();
        my_win.add(4).unwrap();
        my_win.add(6).unwrap();
        let sum = my_win.sum().unwrap();
        assert_eq!(sum, 12);
    }

    #[test]
    fn no_incomplete_sums() {
        let mut my_win = Window::new(3);
        my_win.add(2).unwrap();
        my_win.add(4).unwrap();
        if let Ok(_) = my_win.sum() {
            panic!("incomplete measurement windows shouldn't have valid sums!")
        }
    }
}
