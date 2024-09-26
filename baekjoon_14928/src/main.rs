use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().chars();

    let mut res = 0;
    for i in input {
        res = (res * 10 + (i as i32 - '0' as i32)) % 20000303;
    }

    stdout().write_all(res.to_string().as_bytes()).unwrap();
}
