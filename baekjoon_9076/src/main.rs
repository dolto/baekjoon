use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let count = input.next().unwrap().parse::<usize>().unwrap();

    let mut member = [0; 5];

    for _ in 0..count {
        for i in 0..5 {
            member[i] = input.next().unwrap().parse().unwrap();
        }
        member.sort();
        let smax = member[3];
        let smin = member[1];

        if smax - smin >= 4 {
            stdout().write_all("KIN\n".as_bytes()).unwrap();
        } else {
            let total: i32 = member[1..=3].iter().sum();
            stdout().write_all(format!("{total}\n").as_bytes()).unwrap();
        }
    }
}
