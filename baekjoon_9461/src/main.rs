use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|i| i.parse::<usize>());

    let mut writer = BufWriter::new(stdout().lock());

    let mut log = Vec::with_capacity(100);
    log.push(1);
    log.push(1);
    log.push(1);
    log.push(2);
    log.push(2);
    while let Some(n) = input.next() {
        let res;
        if log.len() <= n {
            for i in log.len() - 1..n {
                padoban(i, &mut log);
            }
        }
        res = log[n - 1];
        writer.write(format!("{res}\n").as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}

fn padoban(n: usize, log: &mut Vec<usize>) {
    let x = log[n];
    log.push(x + log[n - 4]);
}
