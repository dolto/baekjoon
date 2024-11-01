use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let mut max = 0;
    let mut temp;
    for _ in 0..2 {
        temp = 0;
        for _ in 0..4 {
            temp += input.next().unwrap();
        }
        max = max.max(temp);
    }

    stdout().write(max.to_string().as_bytes()).unwrap();
}
