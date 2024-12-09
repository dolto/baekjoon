use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let mut write = BufWriter::new(stdout().lock());

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if a == 0 && b == 0 {
            continue;
        }
        if a == 0 || b == 0 {
            writeln!(write, "neither").unwrap();
        } else if a % b == 0 {
            writeln!(write, "multiple").unwrap();
        } else if b % a == 0 {
            writeln!(write, "factor").unwrap();
        } else {
            writeln!(write, "neither").unwrap();
        }
    }
    write.flush().unwrap();
}
