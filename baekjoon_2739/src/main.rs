use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut bw = BufWriter::new(stdout().lock());
    for i in 1..=9 {
        writeln!(bw, "{} * {} = {}", input, i, input * i).unwrap();
    }

    bw.flush().unwrap();
}
