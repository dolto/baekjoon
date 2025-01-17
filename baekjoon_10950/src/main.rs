use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let t = input.next().unwrap();
    let mut bw = BufWriter::new(stdout().lock());

    (0..t).for_each(|_| writeln!(bw, "{}", input.next().unwrap() + input.next().unwrap()).unwrap());

    bw.flush().unwrap();
}
