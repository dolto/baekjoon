use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().chars();
    let mut hashmap = HashMap::new();

    while let Some(c) = input.next() {
        *hashmap.entry(c).or_insert(0) += 1;
    }

    let mut odd = false;
    let mut even = false;
    for (_, &v) in hashmap.iter() {
        if v % 2 == 0 {
            even = true;
        } else {
            odd = true;
        }
    }

    if odd && !even {
        stdout().write_all("1".as_bytes()).unwrap();
    } else if !odd && even {
        stdout().write_all("0".as_bytes()).unwrap();
    } else {
        stdout().write_all("0/1".as_bytes()).unwrap();
    }
}
