use std::fs;

pub fn load_inputs(dataset: &str) -> std::io::Result<String> {
  let file = format!("./inputs/{}.txt", dataset);
  fs::read_to_string(file)
}

// Get a number or die trying
pub fn usize_or_die(s: &str) -> usize {
  usize::from_str_radix(s, 10).unwrap()
}

pub fn i32_or_die(s: &str) -> i32 {
  i32::from_str_radix(s, 10).unwrap()
}
