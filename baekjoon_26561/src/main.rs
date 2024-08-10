use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<u64>());
    let mut writer = BufWriter::new(stdout().lock());

    let count = input.next().unwrap();
    for _ in 0..count {
        let original = input.next().unwrap();
        let time = input.next().unwrap();
        let change = (time / 4) - (time / 7);
        writer
            .write_all(format!("{}\n", original + change).as_bytes())
            .unwrap();
    }
    writer.flush().unwrap();
}
