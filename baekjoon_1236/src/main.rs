use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let y = input.next().unwrap().parse::<usize>().unwrap();
    let x = input.next().unwrap().parse::<usize>().unwrap();
    let mut cols = vec![false; y];
    let mut rows = vec![false; x];

    for (col, s) in input.enumerate() {
        for (row, c) in s.chars().enumerate() {
            if c == 'X' {
                cols[col] = true;
                rows[row] = true;
            }
        }
    }

    let answer = cols
        .into_iter()
        .filter(|b| !b)
        .count()
        .max(rows.into_iter().filter(|b| !b).count());

    stdout().write(answer.to_string().as_bytes()).unwrap();
}
