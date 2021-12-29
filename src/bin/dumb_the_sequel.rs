fn main() {
    // deep cloning nested vecs?
    // let original = vec![vec![1,2,3], vec![0,5,5]];
    // let mut dupe = original.clone();
    // dupe[0].push(990);
    // dbg!(&original);
    // dbg!(&dupe);
    // // can I string from char?
    // let mut sss = String::from('s');
    // sss.push('s');
    // sss.push_str("s");
    // dbg!(sss);

    // Right, first off let's figure out how to handle binary numbers around
    // here. By way of confession: I think I probably should have been using u32
    // for some of these problems instead of just usize for everything, but I
    // was initially unsure of when I'd need to use something as an index into a
    // collection later, so I ended up just taking the easy route.
    let some_num = u128::from_str_radix("A0016C880162017C3686B18A3D4780", 16).unwrap();
    println!("and now it's binary! {:b}", some_num);
    // Right!! First thing I've learned here is that we can't just turn the
    // input into a number and work from there! it's too big!!! I didn't really
    // understand the scale there. So we'll need to take a bite at a time.

    // Hey, does .take() consume an iterator?
    // let mut chars = "A0016C880162017C3686B18A3D4780".chars();
    // let first = chars.take(3);
    // let second = chars.take(3);
    // IT DOES. sigh.

    // let mut stack = vec![Yo{encoded_size: 9}, Yo{encoded_size: 20}];
    // let child = stack.pop().unwrap();
    // let parent = stack.last_mut().unwrap();
    // parent.encoded_size += child.encoded_size + 8;
    // dbg!(&parent);

    // Boxes use Move semantics, right?
    // let thing = Box::new("hi");
    // println!("the stuff is here: {:?}", thing);
    // let elsewhere = thing;
    // println!("the stuff is here: {}", thing);
    // yes they do.
    // let thingref = &thing;
    // println!("where's the stuff? {:?}", thingref);
    // println!("where's the original stuff? {:?}", thing);

    // OK, time to learn the basics of recursive data structures I guess.

    let one = Snails::Pair(
        Box::new(Snails::Regular(8)),
        Box::new(Snails::Regular(4))
    );
    let two = Snails::Pair(
        Box::new(one),
        Box::new(Snails::Regular(9))
    );
    let three = Snails::Pair(
        Box::new(Snails::Regular(2)),
        Box::new(two)
    );
    let mut outer = SnailfishNumber(Box::new(three));
    // Annoying!!!
    // dbg!(&three);
    // if let Snails::Pair(_, right) = &mut three {
    //     dbg!(right);
    //     if let Snails::Pair(left, _) = right {
    //         dbg!(left);
    //     }
    // }
    let SnailfishNumber(third) = &mut outer;
    match **third {
        Snails::Regular(ref num) => {
            println!("regular {}", num);
        },
        Snails::Pair(ref l, ref r) => {
            println!("l: {:?} r: {:?}", l, r);
            if let Snails::Pair(ref second_l, ref second_r) = **r {
                dbg!(second_l);
            }
        }
    }
    dbg!(third);
}

struct SnailfishNumber(Box<Snails>);

#[derive(Debug)]
enum Snails {
    Regular(u32),
    Pair(Box<Snails>, Box<Snails>),
}

#[derive(Debug)]
struct Yo {
    encoded_size: usize,
}
