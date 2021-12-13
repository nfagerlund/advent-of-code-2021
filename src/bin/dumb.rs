use std::collections::HashSet;

fn main() {
    println!("hi");
    // let inputs = [18, 9];
    // inputs.sort();
    // println!("sorted inputs: {:#?}", &inputs);
    // let thing: Vec<i32> = (inputs[0]..=inputs[1]).collect();
    // println!("so:\n{:#?}", thing);
        // What I learned: I think I was just experimenting and trying to find
        // something that'd work for day 5.1.

    // println!("beyond the veil: {}", inputs[3]);
        // What I learned: out-of-bounds array access panics on runtime, BUT, if
        // the compiler KNOWS that a particular access will ALWAYS panic (like
        // this one, which uses a constant expression), it'll just call it a
        // compiler error instead and stop you. (configurable with
        // `#[deny(unconditional_panic)]`)

    // testing HashSet
    let mut first = HashSet::new();
    let mut second = HashSet::new();
    let mut third = HashSet::new();
    first.insert('a');
    first.insert('c');
    second.insert('c');
    second.insert('a');
    third.insert('b');
    println!("first == second: {}", first == second);
    println!("first == third: {}", first == third);
        // what I learned: HashSet implements equality like I'd expect.
    println!("pow!! {}", 10usize.pow(0));

    println!("wrappy? {:?}", 0usize.checked_sub(1));
}
