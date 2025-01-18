use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let (n, x) = (input.next().unwrap() as usize, input.next().unwrap());

    let mut v = Vec::with_capacity(n);
    (0..n).for_each(|_| v.push(input.next().unwrap()));
    v = v.into_iter().filter(|i| *i < x).collect();

    let mut bw = BufWriter::new(stdout().lock());

    for i in v {
        write!(bw, "{i} ").unwrap();
    }

    bw.flush().unwrap();
}
