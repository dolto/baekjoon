use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let mut wirter = BufWriter::new(stdout().lock());

    while let Some(left) = input.next() {
        let right = input.next().unwrap();
        wirter
            .write_all(format!("{}\n", left + right).as_bytes())
            .unwrap();
    }
    wirter.flush().unwrap();
}
