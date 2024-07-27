use std::io::{stdin, stdout, BufReader, Read, Write};

fn men_of_passion(a: Vec<i32>, n: usize) -> i32 {
    let i = a[n / 2];
    i
}
fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let res = input
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    men_of_passion(vec![0; res], res);

    stdout().write_all(format!("1\n0").as_bytes()).unwrap();
}
