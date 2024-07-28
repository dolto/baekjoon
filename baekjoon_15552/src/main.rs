use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let count = input.next().unwrap().trim().parse().unwrap();

    let mut output = String::new();

    for _ in 0..count {
        let left: i32 = input.next().unwrap().parse().unwrap();
        let right: i32 = input.next().unwrap().parse().unwrap();
        output.push_str(format!("{}\n", left + right).as_str());
    }
    stdout().write_all(output.as_bytes()).unwrap();
}
