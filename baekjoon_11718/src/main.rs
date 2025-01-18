use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::with_capacity(100 * 100);
    stdin().read_to_string(&mut input).unwrap();
    stdout().write(input.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
