use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let count = input.next().unwrap().parse().unwrap();
    let mut log = Vec::with_capacity(count);

    for i in 1..=count {
        log.push(solved_log(i as i64, &log));
    }
    stdout()
        .write_all(format!("{}", log[count - 1]).as_bytes())
        .unwrap();
}

fn solved_log(n: i64, log: &Vec<i64>) -> i64 {
    if n == 1 {
        1
    } else if n == 2 {
        2
    } else {
        (log[n as usize - 3] + log[n as usize - 2]) % 10007
    }
}
