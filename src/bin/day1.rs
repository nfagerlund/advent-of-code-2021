use std::fs;

fn load_inputs(dataset: &str) -> std::io::Result<String> {
    let file = format!("./inputs/{}.txt", dataset);
    fs::read_to_string(file)
}

// Count how many times the depth *increases*. Doesn't matter how much it
// increases by, doesn't matter how many measurements there were.
// Input is numbers separated by newlines.
fn main() {
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
