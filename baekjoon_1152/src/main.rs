use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(1_000_001);
    stdin().read_line(&mut input).unwrap();

    write!(
        stdout(),
        "{}",
        input.trim().split_ascii_whitespace().count()
    )
    .unwrap();
    stdout().flush().unwrap();
}
