use std::io::{stdin, stdout, Write};

const A: u8 = b'A' - 10;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_ascii_whitespace();
    let mut num = input.next().unwrap();

    while let Some(temp) = num.strip_prefix("0") {
        num = temp;
    }
    if num.is_empty() {
        num = "0";
    }
    let num = num.bytes().collect::<Vec<u8>>();
    let mut size = 0;
    let base = input.next().unwrap().parse::<u64>().unwrap();

    let mut answer = 0;

    for b in num.into_iter().rev() {
        let u = (if b >= b'A' { b - A } else { b - b'0' }) as u64;
        let u = u * base.pow(size);
        answer += u;
        size += 1;
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
