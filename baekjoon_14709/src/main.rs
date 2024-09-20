use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let count = input.next().unwrap();

    let mut hashset = HashSet::with_capacity(count);
    for _ in 0..count {
        let mut head = [input.next().unwrap(), input.next().unwrap()];
        head.sort();
        hashset.insert(head);
    }
    if hashset.len() == 3
        && hashset.contains(&[1, 3])
        && hashset.contains(&[1, 4])
        && hashset.contains(&[3, 4])
    {
        stdout()
            .write_all(format!("Wa-pa-pa-pa-pa-pa-pow!").as_bytes())
            .unwrap();
    } else {
        stdout()
            .write_all(format!("Woof-meow-tweet-squeek").as_bytes())
            .unwrap();
    }
}
