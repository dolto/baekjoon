use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace();

    let mut writer = BufWriter::new(stdout().lock());

    let mut input = input.skip(1);
    while let Some(c) = input.next() {
        let itr: i32 = c.parse().unwrap();
        let temp = input.next().unwrap();
        for c in temp.bytes() {
            for _ in 0..itr {
                writer.write(&[c]).unwrap();
            }
        }
        writer.write("\n".as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}
