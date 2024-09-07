use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<i64>().unwrap();

    let mut count = 0;

    while input > 0 {
        input -= count * 2 + 1;
        count += 1;
    }
    if input < 0 {
        count -= 1;
    }
    stdout()
        .write_all(format!("The largest square has side length {count}.").as_bytes())
        .unwrap();
}
