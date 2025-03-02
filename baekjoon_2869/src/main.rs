use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (a, b, mut v) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    v -= a;
    let mut answer = 1;
    if v > 0 {
        let ab = a - b;
        answer += v / ab - 1;
        v -= ab * answer - b;
        // println!("{v}");
        if v > b {
            answer += 2;
        } else {
            answer += 1;
        }
    }

    // println!("{v}");

    write!(stdout(), "{}", answer).unwrap();
    stdout().flush().unwrap();
}
