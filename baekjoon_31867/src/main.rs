use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let _n = input.next().unwrap().parse::<i32>().unwrap();
    let value = input.next().unwrap().chars();

    let mut jjack = 0;
    let mut hholl = 0;
    for temp in value.into_iter() {
        let temp = temp.to_string().parse::<i32>().unwrap();
        if temp % 2 == 0 {
            jjack += 1;
        } else {
            hholl += 1;
        }
    }

    if jjack > hholl {
        stdout().write_all("0".as_bytes()).unwrap();
    } else if hholl > jjack {
        stdout().write_all("1".as_bytes()).unwrap();
    } else {
        stdout().write_all("-1".as_bytes()).unwrap();
    }
}
