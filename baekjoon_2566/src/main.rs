use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::with_capacity(81 * 4);
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let mut answer = 0;
    let (mut x, mut y) = (1, 1);
    for i in 1..=9 {
        for j in 1..=9 {
            let ele = input.next().unwrap();
            if answer < ele {
                answer = ele;
                x = i;
                y = j;
            }
        }
    }

    write!(stdout(), "{}\n{} {}", answer, x, y).unwrap();
    stdout().flush().unwrap();
}
