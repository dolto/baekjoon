use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    stdout()
        .write(format!("{}??!", input.trim()).as_bytes())
        .unwrap();
}
