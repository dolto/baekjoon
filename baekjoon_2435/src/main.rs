use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let k = input.next().unwrap().parse::<usize>().unwrap();

    let mut days = Vec::with_capacity(n);
    for i in input {
        let i: i32 = i.parse().unwrap();
        days.push(i);
    }

    let mut max = i32::MIN;
    for i in 0..=(n - k) {
        let mut temp = 0;
        for j in 0..k {
            temp += days[i + j];
        }
        max = max.max(temp);
    }

    stdout().write_all(max.to_string().as_bytes()).unwrap();
}
