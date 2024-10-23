use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let count = input.next().unwrap().parse::<usize>().unwrap();
    let input = input.next().unwrap().chars();

    let mut output = Vec::with_capacity(count);
    for c in input {
        if c == 'l' {
            output.push('L' as u8);
        } else {
            output.push('i' as u8);
        }
    }

    stdout().write_all(output.as_slice()).unwrap();
}
