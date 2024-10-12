use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());

    let mut cow = Vec::with_capacity(input.next().unwrap());
    let size = input.next().unwrap();
    let mut answer = 0;
    while let Some(moo) = input.next() {
        cow.push(moo);
    }
    cow.sort();
    for fi in 0..(cow.len() - 1) {
        for ei in ((fi + 1)..cow.len()).rev() {
            if cow[fi] + cow[ei] <= size {
                answer += 1;
            }
        }
    }
    stdout().write_all(answer.to_string().as_bytes()).unwrap();
}
