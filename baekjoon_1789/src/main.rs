use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<i64>().unwrap();

    let mut count = 1;
    while input > 0 {
        input -= count;
        count += 1;
    }

    let answer = count - if input == 0 { 1 } else { 2 };

    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
