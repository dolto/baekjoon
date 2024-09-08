use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, StdoutLock, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .trim()
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut writer = BufWriter::new(stdout().lock());
    let len = input.next().unwrap();
    let count = input.next().unwrap();
    let mut hashset = HashSet::with_capacity(count * len);

    solve(&mut writer, len, 1, String::new(), count, &mut hashset);

    let mut vec = hashset.into_iter().collect::<Vec<String>>();
    vec.sort();
    vec.pop();
    for v in vec.iter() {
        writer.write(format!("{v}\n").as_bytes()).unwrap();
    }
    for _ in 0..count - 1 {
        writer.write((len.to_string() + " ").as_bytes()).unwrap();
    }
    writer.write((len.to_string()).as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn solve(
    writer: &mut BufWriter<StdoutLock>,
    len: usize,
    now: usize,
    result: String,
    left: usize,
    log: &mut HashSet<String>,
) {
    if left == 0 {
        log.insert(format!("{}", result.trim()));
    } else if now <= len {
        solve(
            writer,
            len,
            now,
            result.clone() + " " + now.to_string().as_str(),
            left - 1,
            log,
        );
        solve(
            writer,
            len,
            now + 1,
            result.clone() + " " + now.to_string().as_str(),
            left - 1,
            log,
        );
        solve(writer, len, now + 1, result.clone(), left, log);
    }
}
