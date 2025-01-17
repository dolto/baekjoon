use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap() / 4;

    let mut bw = BufWriter::new(stdout().lock());
    for _ in 0..input {
        write!(bw, "long ").unwrap();
    }
    write!(bw, "int").unwrap();

    bw.flush().unwrap();
}
