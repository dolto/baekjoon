use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let chicken = input.next().unwrap();
    let beef = input.next().unwrap();
    let pasta = input.next().unwrap();

    let want_chicken = input.next().unwrap();
    let want_beef = input.next().unwrap();
    let want_pasta = input.next().unwrap();

    let res =
        (want_chicken - chicken).max(0) + (want_beef - beef).max(0) + (want_pasta - pasta).max(0);
    stdout().write_all(format!("{res}").as_bytes()).unwrap();
}
