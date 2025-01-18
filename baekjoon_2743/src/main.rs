use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(100);
    stdin().read_line(&mut input).unwrap();

    stdout()
        .write(input.trim().len().to_string().as_bytes())
        .unwrap();
    stdout().flush().unwrap();
}
