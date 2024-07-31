use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let cost: i32 = input.next().unwrap().trim().parse().unwrap();
    let count: i32 = input.next().unwrap().trim().parse().unwrap();
    let dongsu: i32 = input.next().unwrap().trim().parse().unwrap();

    let perant = (cost * count - dongsu).max(0);

    stdout().write_all(format!("{perant}").as_bytes()).unwrap();
}
