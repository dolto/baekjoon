use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let mut bw = BufWriter::new(stdout().lock());

    for i in 1..=input.next().unwrap() {
        writeln!(
            bw,
            "Case #{}: {}",
            i,
            input.next().unwrap() + input.next().unwrap()
        )
        .unwrap();
    }

    bw.flush().unwrap();
}
