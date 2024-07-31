use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let req: i32 = input.trim().parse().unwrap();

    let res = req - req / 11;
    stdout().write_all(format!("{res}").as_bytes()).unwrap();
}
