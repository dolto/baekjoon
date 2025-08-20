use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let percent = input.trim().parse::<i32>().unwrap() / 100;

    let answer1 = percent * 78;
    let answer2 = percent * 100 - (percent * 20 / 100 * 22);

    write!(stdout(), "{} {}", answer1, answer2).unwrap();
    stdout().flush().unwrap();
}
