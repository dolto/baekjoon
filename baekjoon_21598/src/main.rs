use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse().unwrap();
    let mut writer = BufWriter::new(stdout().lock());
    for _ in 0..input {
        writer.write_all("SciComLove\n".as_bytes()).unwrap();
    }
}
