fn main() {
    println!("hi");
    let mut inputs = [18, 9];
    inputs.sort();
    println!("sorted inputs: {:#?}", &inputs);
    let thing: Vec<i32> = (inputs[0]..=inputs[1]).collect();
    println!("so:\n{:#?}", thing);
}
