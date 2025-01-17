use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let mut bw = BufWriter::new(stdout().lock());

    loop {
        let l = input.next().unwrap();
        if l == 0 {
            break;
        }
        let r = input.next().unwrap();
        writeln!(bw, "{}", l + r).unwrap();
    }

    bw.flush().unwrap();
}
