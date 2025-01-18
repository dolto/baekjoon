use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut v = (0..=n).collect::<Vec<usize>>();

    (0..m).for_each(|_| {
        let (l, r) = (input.next().unwrap(), input.next().unwrap());
        v.swap(l, r);
    });

    let mut bw = BufWriter::new(stdout().lock());
    (1..=n).for_each(|i| {
        write!(bw, "{} ", v[i]).unwrap();
    });
    bw.flush().unwrap();
}
