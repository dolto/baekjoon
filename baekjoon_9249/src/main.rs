use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut a: Vec<char> = input.next().unwrap().chars().collect();
    let mut b: Vec<char> = input.next().unwrap().chars().collect();

    if a.len() < b.len() {
        let temp = b;
        b = a;
        a = temp;
    }

    let mut max = 0;
    for i in 0..b.len() {
        if b[i] == a[max] {
            max += 1;
        } else {
            max = 0;
        }
    }
}
