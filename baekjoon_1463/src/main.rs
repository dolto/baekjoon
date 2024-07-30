use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let x: usize = input.trim().parse().unwrap();
    let mut log = Vec::with_capacity(x);

    for i in 0..=x {
        log.push(make_one(i, &log));
    }
    stdout()
        .write_all(format!("{}", log[x]).as_bytes())
        .unwrap();
}

fn make_one(i: usize, log: &Vec<usize>) -> usize {
    if i <= 1 {
        0
    } else if i == 2 || i == 3 {
        1
    } else {
        if i % 2 == 0 && i % 3 == 0 {
            1 + log[i / 2].min(log[i / 3].min(log[i - 1]))
        } else if i % 2 == 0 {
            // 1 + make_one(i / 2, log).min(make_one(i - 1, log))
            1 + log[i / 2].min(log[i - 1])
        } else if i % 3 == 0 {
            // 1 + make_one(i / 3, log).min(make_one(i - 1, log))
            1 + log[i / 3].min(log[i - 1])
        } else {
            1 + log[i - 1]
        }
    }
}
