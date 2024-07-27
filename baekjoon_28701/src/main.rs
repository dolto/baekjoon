use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();
    let res = input.next().unwrap().trim().parse().unwrap();

    let range = 1..=res;

    let mut n_to_1: i64 = 0;
    let mut n_to_1_pow_3: i64 = 0;
    range.for_each(|n| {
        n_to_1 += n;
        n_to_1_pow_3 += n.pow(3);
    });
    let n_to_1_pow = n_to_1.pow(2);

    stdout()
        .write_all(format!("{n_to_1}\n{n_to_1_pow}\n{n_to_1_pow_3}").as_bytes())
        .unwrap();
}
