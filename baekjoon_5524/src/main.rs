use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().skip(1);

    let mut writer = BufWriter::new(stdout().lock());
    while let Some(s) = input.next() {
        let s = s.to_lowercase() + "\n";
        writer.write(s.as_bytes()).unwrap();
    }
}
