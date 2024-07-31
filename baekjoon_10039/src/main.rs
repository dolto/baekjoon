use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut total = 0;
    for _ in 0..5 {
        total += input.next().unwrap().trim().parse::<i32>().unwrap().max(40);
    }
    stdout()
        .write_all(format!("{}", total / 5).as_bytes())
        .unwrap();
}
