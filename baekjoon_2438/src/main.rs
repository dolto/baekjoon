use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();
    let mut bw = BufWriter::new(stdout().lock());

    let mut stars = String::with_capacity(input);

    (0..input).for_each(|_| {
        stars.push_str("*");
        writeln!(bw, "{stars}").unwrap();
    });

    bw.flush().unwrap();
}
