use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();
    let mut answer = 1;
    let mut max = 1;
    loop {
        if max < input {
            max += 6 * answer;
            answer += 1;
        } else {
            break;
        }
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
}
