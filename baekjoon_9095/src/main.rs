use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    let mut log = Vec::with_capacity(10);

    for i in 1..=10 {
        log.push(make_solved(i, &log));
    }
    let count = input.next().unwrap().parse().unwrap();
    for _ in 0..count {
        let n: usize = input.next().unwrap().parse().unwrap();
        writer
            .write_all(format!("{}\n", log[n - 1]).as_bytes())
            .unwrap();
    }
    writer.flush().unwrap();
}

fn make_solved(n: i32, log: &Vec<i32>) -> i32 {
    if n == 1 {
        1
    } else if n == 2 {
        2
    } else if n == 3 {
        4
    } else {
        log[n as usize - 2] + log[n as usize - 3] + log[n as usize - 4]
    }
}
