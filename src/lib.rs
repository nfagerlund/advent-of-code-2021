use std::fs;

pub fn load_inputs(dataset: &str) -> std::io::Result<String> {
  let file = format!("./inputs/{}.txt", dataset);
  fs::read_to_string(file)
}
