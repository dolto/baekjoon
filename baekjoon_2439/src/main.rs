use std::io::{stdin, stdout, BufWriter, Write};

const EMPTY: u8 = ' ' as u8;
const STAR: u8 = '*' as u8;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap() + 1;
    let mut bw = BufWriter::new(stdout().lock());

    let mut bytes = vec![EMPTY; input];
    let size = bytes.len();
    bytes[size - 1] = '\n' as u8;

    (2..=input).for_each(|i| {
        bytes[size - i] = STAR;
        bw.write(&bytes).unwrap();
    });

    bw.flush().unwrap();
}
