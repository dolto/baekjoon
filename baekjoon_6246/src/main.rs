use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut slot = vec![true; input.next().unwrap()];
    input.next();

    while let (Some(l), Some(i)) = (input.next(), input.next()) {
        for index in ((l - 1)..slot.len()).step_by(i) {
            slot[index] = false;
        }
    }

    let empty = slot.iter().filter(|a| **a).count();
    stdout().write(empty.to_string().as_bytes()).unwrap();
}
