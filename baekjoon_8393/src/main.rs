use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();
    if n % 2 == 0 {
        let count = n / 2;
        stdout()
            .write_all(format!("{}", (n + 1) * count).as_bytes())
            .unwrap();
    } else {
        let count = n / 2 + 1;
        stdout()
            .write_all(format!("{}", n * count).as_bytes())
            .unwrap();
    }
}
