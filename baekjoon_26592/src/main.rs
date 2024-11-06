use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let test_case = input.next().unwrap().parse().unwrap();
    let mut bw = BufWriter::new(stdout().lock());

    for _ in 0..test_case {
        let area = input.next().unwrap().parse::<f64>().unwrap() * 2.;
        let base = input.next().unwrap().parse::<f64>().unwrap();

        let answer = area / base;
        writeln!(bw, "The height of the triangle is {:.2} units", answer).unwrap();
    }

    bw.flush().unwrap();
}
