use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input.split_ascii_whitespace().skip(1);

    let mut bw = BufWriter::new(stdout().lock());
    for s in input {
        let chs = s.chars().collect::<Vec<char>>();
        writeln!(bw, "{}{}", chs[0], chs[s.len() - 1]).unwrap();
    }
    bw.flush().unwrap();
}
