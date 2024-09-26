use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let res = input.split("DKSH").count() - 1;
    stdout().write_all(res.to_string().as_bytes()).unwrap();
}
