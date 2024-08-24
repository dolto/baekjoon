use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let (a, b, c, d, e, f) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let (temp1, temp2) = ((c * d - a * f), (b * d - a * e));
    let y = if temp1 != 0 && temp2 != 0 {
        temp1 / temp2
    } else {
        0
    };

    let (temp1, temp2) = ((c * e - b * f), (a * e - b * d));
    let x = if temp1 != 0 && temp2 != 0 {
        temp1 / temp2
    } else {
        0
    };

    // println!("{} = {c}", a * x + b * y);
    // println!("{} = {f}", d * x + e * y);

    stdout().write(format!("{} {}", x, y).as_bytes()).unwrap();
}
