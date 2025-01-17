use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let mut v = Vec::with_capacity(3);
    for i in input {
        v.push(i);
    }
    v.sort();

    let mut count = 1;
    let mut same = 0;
    let mut last = 0;
    for i in v {
        if last == i {
            count += 1;
            same = i;
        }
        last = i;
    }

    if count > 1 {
        write!(
            stdout(),
            "{}",
            10_i32.pow(count + 1) + same * 10_i32.pow(count)
        )
        .unwrap();
    } else {
        write!(stdout(), "{}", last * 100).unwrap();
    }

    stdout().flush().unwrap();
}
