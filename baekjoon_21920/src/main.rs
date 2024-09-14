use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let count = input.next().unwrap();
    let mut log = Vec::with_capacity(count);
    for _ in 0..count {
        log.push(input.next().unwrap());
    }

    let x = input.next().unwrap();
    solved(log, x);
    // for i in 2..=100 {
    //     for j in 2..=100 {
    //         solved(vec![i, i], j);
    //     }
    // }
}

fn solved(log: Vec<usize>, x: usize) {
    let log = log.iter().filter(|&i| {
        let mut temp = *i;
        let mut x = x;
        while temp != 0 {
            let t = temp;
            temp = x % temp;
            x = t;
        }
        x == 1
    });

    let mut count = 0;
    let mut total = 0.;
    for i in log {
        count += 1;
        total += *i as f64;
    }

    let result = if count > 0 { total / count as f64 } else { 0. };
    stdout()
        .write_all(format!("{:.6}\n", result).as_bytes())
        .unwrap();
}
