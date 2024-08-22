use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>())
        .skip(1);
    let mut expect = 1;
    let mut result = 0;

    while let Some(number) = input.next() {
        if number != expect {
            result += 1;
        }
        expect += 1;
    }

    stdout()
        .write_all(format!("{}", result).as_bytes())
        .unwrap();
}
