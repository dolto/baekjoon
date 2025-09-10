use std::io::{Read, Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let (n, s) = (input.next().unwrap() as usize, input.next().unwrap());

    let mut vec = Vec::with_capacity(n);
    let (mut start, mut end) = (0, 0);
    let mut sum = 0;
    let mut answer = usize::MAX;

    for i in input {
        vec.push(i);

        end += 1;
        sum += i;
        while sum >= s {
            answer = answer.min(end - start);
            sum -= vec[start];
            start += 1;
        }
    }

    if answer == usize::MAX {
        answer = 0;
    }

    writeln!(stdout(), "{}", answer).unwrap();
    stdout().flush().unwrap();
}
