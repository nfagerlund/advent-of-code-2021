use std::collections::VecDeque;
use std::fs;
use std::fmt;

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
    let mut last_sum: Option<i32> = None;
    let mut increases = 0;
    let mut measurer = WindowMgr::new(3);
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

    // RIGHT.
    for sounding in soundings_iter {
        let next_sum = measurer.measure_and_extract_sum(sounding);
        // IF we actually have complete windows to work with, then compare them.
        if let Some(previous) = last_sum {
            if let Some(next) = next_sum {
                if next > previous {
                    increases += 1;
                }
            }
        }
        // Regardless, update the last sum (which might still be nothing during the spin-up.)
        last_sum = next_sum;
    }

    println!("Depth increased {} times (sliding window of 3)", increases);
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

pub struct WindowMgr {
    capacity: usize,
    window_buffer: VecDeque<Window>,
}

impl WindowMgr {
    pub fn new(capacity: usize) -> WindowMgr {
        WindowMgr {
            capacity,
            window_buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn measure(&mut self, measurement: i32) -> Option<Window> {
        // Once per measurement, before measuring, fill up to one empty slots in the buffer.
        if self.window_buffer.len() < self.capacity {
            self.window_buffer.push_back(Window::new(self.capacity));
        }
        // Add new measurement to every current window:
        for window in self.window_buffer.iter_mut() {
            // Uhhhh panicking here is a bit fast and loose, but, it's advent of
            // code so that's more or less what I want.
            window.add(measurement).unwrap();
        }
        // If the first window is full up, move it out of ourselves and return it.
        // ...Again, unwrap seems proportionate for the time being. Should
        // always be at least one window here by this point.
        // Anyway, after doing this, there'll be a gap to fill next time.
        if self.window_buffer.front().unwrap().complete() {
            Some(self.window_buffer.pop_front().unwrap())
        } else {
            None
        }
    }

    pub fn measure_and_extract_sum(&mut self, measurement: i32) -> Option<i32> {
        match self.measure(measurement) {
            None => None,
            // vv :| I don't really like this, but I kind of painted myself into
            // a corner by mixing Options and Results like this.
            Some(win) => Some(win.sum().unwrap()),
        }
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

    #[test]
    fn window_mgr_basics() {
        let mut my_mgr = WindowMgr::new(3);
        if let Some(_) = my_mgr.measure(4) {
            panic!("shouldn't be returning anything yet");
        }
        if let Some(_) = my_mgr.measure(4) {
            panic!("shouldn't be returning anything yet");
        }
        // Finally ready for a sum:
        let win = my_mgr.measure(5).unwrap();
        assert_eq!(win.sum().unwrap(), 13);
        // And another:
        let win = my_mgr.measure(6).unwrap();
        assert_eq!(win.sum().unwrap(), 15);
        let sum = my_mgr.measure_and_extract_sum(9).unwrap();
        assert_eq!(sum, 20);
    }
}
