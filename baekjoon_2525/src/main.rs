use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace().flat_map(|s| s.parse::<i32>());
    let (mut h, mut m) = (input.next().unwrap(), input.next().unwrap());
    m += input.next().unwrap();

    if m >= 60 {
        h += m / 60;
        m = m % 60;
        if h >= 24 {
            h %= 24;
        }
    }

    stdout().write(format!("{h} {m}").as_bytes()).unwrap();
}
