use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let count = input.next().unwrap().trim().parse().unwrap();
    let mut log = Vec::with_capacity(41);
    for i in 0..=40 {
        log.push(fibonacci(i, &log));
    }
    for _ in 0..count {
        let res: usize = input.next().unwrap().trim().parse().unwrap();
        if res == 0 {
            writer.write_all("1 0\n".as_bytes()).unwrap();
        } else if res == 1 {
            writer.write_all("0 1\n".as_bytes()).unwrap();
        } else {
            writer
                .write_all(format!("{} {}\n", log[res - 1], log[res]).as_bytes())
                .unwrap();
        }
    }
    writer.flush().unwrap();
}

fn fibonacci(i: usize, log: &Vec<usize>) -> usize {
    if i == 0 {
        0
    } else if i == 1 {
        1
    } else if i > 1 {
        log[i - 1] + log[i - 2]
    } else {
        fibonacci(i - 1, log) + fibonacci(i - 2, log)
    }
}
