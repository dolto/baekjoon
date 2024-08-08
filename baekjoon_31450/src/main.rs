use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());
    let medal = input.next().unwrap();
    let children = input.next().unwrap();

    if medal % children == 0 {
        stdout().write_all("Yes".as_bytes()).unwrap();
    } else {
        stdout().write_all("No".as_bytes()).unwrap();
    }
}
