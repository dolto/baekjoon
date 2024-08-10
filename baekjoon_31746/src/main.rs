use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: i64 = input.trim().parse().unwrap();

    let res = "SciComLove".to_string();

    if count % 2 == 0 {
        stdout().write_all(res.as_bytes()).unwrap();
    } else {
        stdout()
            .write_all(res.chars().rev().collect::<String>().as_bytes())
            .unwrap();
    }
}
