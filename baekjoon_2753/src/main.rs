use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    if (input % 4 == 0 && input % 100 != 0) || (input % 400 == 0) {
        stdout().write("1".as_bytes()).unwrap();
    } else {
        stdout().write("0".as_bytes()).unwrap();
    }
}
