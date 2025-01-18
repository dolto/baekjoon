use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    write!(stdout(), "{}", input.bytes().next().unwrap()).unwrap();

    stdout().flush().unwrap();
}
