use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input: i32 = input.trim().parse().unwrap();

    let mut base = 10;

    while input >= base {
        let tinput = input % base;
        if tinput >= base / 2 {
            input += base - tinput;
        } else {
            input -= tinput;
        }
        base *= 10;
    }

    stdout().write(input.to_string().as_bytes()).unwrap();
}
