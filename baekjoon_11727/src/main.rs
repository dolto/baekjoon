use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();

    let mut log = Vec::with_capacity(1000);
    log.push(0);
    log.push(1);
    log.push(3);

    for i in 3..=1000 {
        log.push(((log[i - 2] * 2) % 10007 + log[i - 1]) % 10007);
    }

    stdout()
        .write_all(format!("{}", log[input]).as_bytes())
        .unwrap();
}
