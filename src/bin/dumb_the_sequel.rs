fn main() {
    // deep cloning nested vecs?
    let original = vec![vec![1,2,3], vec![0,5,5]];
    let mut dupe = original.clone();
    dupe[0].push(990);
    dbg!(&original);
    dbg!(&dupe);
    // can I string from char?
    let mut sss = String::from('s');
    sss.push('s');
    sss.push_str("s");
    dbg!(sss);
}
