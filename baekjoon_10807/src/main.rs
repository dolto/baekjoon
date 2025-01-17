use std::{
    collections::HashMap,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let size = input.next().unwrap() as usize;
    let mut hashmap = HashMap::with_capacity(size);

    (0..size).for_each(|_| {
        *hashmap.entry(input.next().unwrap()).or_insert(0) += 1;
    });

    stdout()
        .write(
            hashmap
                .entry(input.next().unwrap())
                .or_insert(0)
                .to_string()
                .as_bytes(),
        )
        .unwrap();
}
