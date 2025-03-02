use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let t = input.next().unwrap();
    let mut bw = BufWriter::new(stdout().lock());
    (0..t).for_each(|_| {
        let (q, d, n, p);
        let mut total = input.next().unwrap();
        q = total / 25;
        total %= 25;
        d = total / 10;
        total %= 10;
        n = total / 5;
        total %= 5;
        p = total;

        writeln!(bw, "{q} {d} {n} {p}").unwrap();
    });

    bw.flush().unwrap();
}
