use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let alen = input.next().unwrap().parse::<usize>().unwrap();
    let candidate = input.next().unwrap();
    let count = candidate.chars().count();
    let mut answer = String::with_capacity(count * alen);
    for _ in 0..alen {
        answer.push_str(candidate);
    }
    stdout().write_all(answer.as_bytes()).unwrap();
}
