use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    let mid = input.len() / 2;
    let left = &input.trim()[0..mid];
    let right = &input.trim()[mid..];
    // println!("{}", left);
    // println!("{}", right);

    let mut a = 0;
    let mut b = 0;

    for l in left.bytes() {
        // println!("{}", l);
        a += l - 48;
    }
    for r in right.bytes() {
        // println!("{}", r);
        b += r - 48;
    }

    if a == b {
        stdout().write("LUCKY".as_bytes()).unwrap();
    } else {
        stdout().write("READY".as_bytes()).unwrap();
    }
}
