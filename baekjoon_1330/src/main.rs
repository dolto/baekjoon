use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_line(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let left: i32 = input.next().unwrap().trim().parse().unwrap();
    let right: i32 = input.next().unwrap().trim().parse().unwrap();

    let res = left.cmp(&right);

    match res {
        std::cmp::Ordering::Equal => {
            writer.write_all("==".as_bytes()).unwrap();
        }
        std::cmp::Ordering::Greater => {
            writer.write_all(">".as_bytes()).unwrap();
        }
        std::cmp::Ordering::Less => {
            writer.write_all("<".as_bytes()).unwrap();
        }
    }
}
