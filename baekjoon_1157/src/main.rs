use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};
const A: u8 = 'A' as u8;
const a: u8 = 'a' as u8;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .bytes()
        .map(|u| if u >= a { u - a } else { u - A });
    let mut hashmap = HashMap::with_capacity(26);

    for b in input {
        *hashmap.entry(b).or_insert(0) += 1;
    }

    let mut max = -1;
    let mut answer = 0;
    for (k, v) in hashmap {
        let k = k + A;
        if max == v {
            answer = '?' as u8;
        } else if max < v {
            max = v;
            answer = k;
        }
    }

    stdout().write(&[answer]).unwrap();
    stdout().flush().unwrap();
}
