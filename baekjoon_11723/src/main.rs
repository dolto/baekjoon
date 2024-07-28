use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_line(&mut input).unwrap();
    // reader.read_line(&mut input).unwrap();
    // let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    // let count = input.next().unwrap().trim().parse().unwrap();
    let count = input.trim().parse().unwrap();
    let mut set = 0;
    for _ in 0..count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut input = input.split_ascii_whitespace();
        let command = input.next().unwrap().trim();
        match command {
            "add" => {
                let param: i32 = input.next().unwrap().trim().parse().unwrap();
                let bit = 1 << param;
                if set & bit == 0 {
                    set += bit;
                }
            }
            "remove" => {
                let param: i32 = input.next().unwrap().trim().parse().unwrap();
                let bit = 1 << param;
                if set & bit > 0 {
                    set -= bit;
                }
            }
            "check" => {
                let param: i32 = input.next().unwrap().trim().parse().unwrap();
                if set & (1 << param) > 0 {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            }
            "toggle" => {
                let param: i32 = input.next().unwrap().trim().parse().unwrap();
                let bit = 1 << param;
                if set & bit > 0 {
                    set -= bit;
                } else {
                    set += bit;
                }
            }
            "all" => {
                set = 0xffffff;
            }
            "empty" => {
                set = 0;
            }
            _ => {}
        }
    }
    // stdout().write_all(output.trim_end().as_bytes()).unwrap();
    writer.flush().unwrap();
}
