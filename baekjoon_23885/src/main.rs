use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().map(|s| s.parse().unwrap());

    let (n, m): (i32, i32) = (input.next().unwrap(), input.next().unwrap());
    let (sx, sy): (i32, i32) = (input.next().unwrap(), input.next().unwrap());
    let (gx, gy) = (input.next().unwrap(), input.next().unwrap());

    if n >= 2 && m >= 2 {
        if sx % 2 == gx % 2 && sy % 2 == gy % 2 {
            stdout().write_all("YES".as_bytes()).unwrap();
        } else if (sx % 2 == gx % 2 && sy % 2 != gy % 2) || (sx % 2 != gx % 2 && sy % 2 == gy % 2) {
            stdout().write_all("NO".as_bytes()).unwrap();
        } else if sx % 2 != gx % 2 && sy % 2 != gy % 2 {
            stdout().write_all("YES".as_bytes()).unwrap();
        }
    } else if sx == gx && sy == gy {
        stdout().write_all("YES".as_bytes()).unwrap();
    } else {
        stdout().write_all("NO".as_bytes()).unwrap();
    }
}
