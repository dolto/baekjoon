use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut input = String::with_capacity(16);
    let mut br = BufReader::new(stdin());
    let mut bw = BufWriter::new(stdout());

    br.read_line(&mut input).unwrap();
    let input = input.trim().chars();

    let mut answer = 0;
    for c in input {
        let n = match c {
            'A' | 'B' | 'C' => 3,
            'D' | 'E' | 'F' => 4,
            'G' | 'H' | 'I' => 5,
            'J' | 'K' | 'L' => 6,
            'M' | 'N' | 'O' => 7,
            'P' | 'Q' | 'R' | 'S' => 8,
            'T' | 'U' | 'V' => 9,
            'W' | 'X' | 'Y' | 'Z' => 10,
            _ => 11,
        };
        answer += n;
    }

    write!(bw, "{}", answer).unwrap();
    bw.flush().unwrap();
}
