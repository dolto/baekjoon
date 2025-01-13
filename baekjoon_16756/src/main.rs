use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let n = input.next().unwrap() as usize;
    let mut answer = i64::MAX;
    let mut list = Vec::with_capacity(n);
    let mut mins = list.clone();
    let mut maxs = list.clone();

    for _ in 0..n {
        list.push(input.next().unwrap());
    }

    for step in 1..n {
        for i in 0..n - step {
            mins[i] = mins[i].min(list[i + step]);
            maxs[i] = maxs[i].max(list[i + step]);
        }
    }

    for i in 0..n {
        answer = answer.min(maxs[i] - mins[i]);
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
}
