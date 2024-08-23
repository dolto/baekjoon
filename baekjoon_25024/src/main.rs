use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let mut writer = BufWriter::new(stdout().lock());

    let count = input.next().unwrap();

    for _ in 0..count {
        let x = input.next().unwrap();
        let y = input.next().unwrap();

        if x < 24 && y < 60 {
            writer.write("Yes ".as_bytes()).unwrap();
        } else {
            writer.write("No ".as_bytes()).unwrap();
        }

        if x > 0 && x <= 12 {
            match x {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                    if y > 0 && y <= 31 {
                        writer.write("Yes\n".as_bytes()).unwrap();
                    } else {
                        writer.write("No\n".as_bytes()).unwrap();
                    }
                }
                4 | 6 | 9 | 11 => {
                    if y > 0 && y <= 30 {
                        writer.write("Yes\n".as_bytes()).unwrap();
                    } else {
                        writer.write("No\n".as_bytes()).unwrap();
                    }
                }
                _ => {
                    if y > 0 && y <= 29 {
                        writer.write("Yes\n".as_bytes()).unwrap();
                    } else {
                        writer.write("No\n".as_bytes()).unwrap();
                    }
                }
            }
        } else {
            writer.write("No\n".as_bytes()).unwrap();
        }
    }
    writer.flush().unwrap();
}
