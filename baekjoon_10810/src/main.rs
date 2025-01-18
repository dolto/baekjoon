use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut boxs = vec![0; n];
    (0..m).for_each(|_| {
        let (l, r, k) = (
            input.next().unwrap() - 1,
            input.next().unwrap(),
            input.next().unwrap(),
        );
        for i in l..r {
            boxs[i] = k;
        }
    });

    let mut bw = BufWriter::new(stdout().lock());
    for i in boxs {
        write!(bw, "{} ", i).unwrap();
    }

    bw.flush().unwrap();
}
