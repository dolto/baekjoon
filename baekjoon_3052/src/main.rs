use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let mut hashset = HashSet::with_capacity(10);

    for i in input {
        hashset.insert(i % 42);
    }

    stdout()
        .write(hashset.len().to_string().as_bytes())
        .unwrap();
}
