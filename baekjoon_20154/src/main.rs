use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut res = VecDeque::with_capacity(input.len());
    let input = input.trim().chars();

    for i in input.into_iter() {
        res.push_back(match i {
            'A' | 'E' | 'F' | 'G' | 'H' | 'K' | 'M' | 'N' => 3,
            'B' | 'D' | 'P' | 'Q' | 'R' | 'T' | 'W' | 'X' | 'Y' => 2,
            _ => 1,
        });
    }

    loop {
        if let Some(i) = res.pop_front() {
            if let Some(j) = res.pop_front() {
                res.push_back((i + j) % 10);
            } else {
                res.push_back(i);
                break;
            }
        }
    }

    if res[0] % 2 == 0 {
        stdout().write_all("You're the winner?".as_bytes()).unwrap();
    } else {
        stdout().write_all("I'm a winner!".as_bytes()).unwrap();
    }
}
