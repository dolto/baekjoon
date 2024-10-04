use std::{
    collections::HashMap,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let count = input.next().unwrap();
    let mut total = 0;
    for _ in 0..count {
        let mut hashmap = HashMap::with_capacity(3);
        let mut t = (0, 0);
        for _ in 0..3 {
            *hashmap.entry(input.next().unwrap()).or_insert(0) += 1;
        }
        for (&k, &v) in hashmap.iter() {
            if t.1 < v {
                t = (k, v);
            } else if t.1 == v && t.0 < k {
                t = (k, v);
            }
        }

        match t.1 {
            1 => {
                total = (t.0 * 100).max(total);
            }
            2 => {
                total = (1000 + t.0 * 100).max(total);
            }
            3 => {
                total = (10000 + t.0 * 1000).max(total);
            }
            _ => {}
        }
    }

    stdout().write(total.to_string().as_bytes()).unwrap();
}
