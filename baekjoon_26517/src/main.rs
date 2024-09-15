use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let k = input.next().unwrap().parse::<i64>().unwrap();
    let (a, b, c, d) = (
        input.next().unwrap().parse::<i64>().unwrap(),
        input.next().unwrap().parse::<i64>().unwrap(),
        input.next().unwrap().parse::<i64>().unwrap(),
        input.next().unwrap().parse::<i64>().unwrap(),
    );

    let left = a * k + b;
    let right = c * k + d;

    if left == right {
        stdout()
            .write_all(format!("Yes {left}").as_bytes())
            .unwrap();
    } else {
        stdout().write_all(format!("No").as_bytes()).unwrap();
    }
}
