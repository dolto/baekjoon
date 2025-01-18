use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let mut index = 0;
    let mut max = i32::MIN;

    for (i, v) in input.enumerate() {
        if max < v {
            index = i + 1;
            max = v;
        }
    }

    write!(stdout(), "{}\n{}", max, index).unwrap();
    stdout().flush().unwrap();
}
