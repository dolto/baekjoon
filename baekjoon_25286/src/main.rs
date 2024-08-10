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
        let y = input.next().unwrap();
        let m = input.next().unwrap();
        writer.write_all(cal_date(y, m).as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}

fn cal_date(y: i32, m: i32) -> String {
    let is_yun = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mut y = y;
    let m = if m - 1 == 0 { 12 } else { m - 1 };
    let d = match m {
        1 | 3 | 5 | 7 | 8 | 10 => 31,
        12 => {
            y -= 1;
            31
        }
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_yun {
                29
            } else {
                28
            }
        }
        _ => 31,
    };

    format!("{y} {m} {d}\n")
}
