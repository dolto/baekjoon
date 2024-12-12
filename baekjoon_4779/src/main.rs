use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<u32>());

    let mut writer = BufWriter::new(stdout().lock());
    for n in input {
        let size = i64::pow(3, n);
        let mut temp = Vec::with_capacity((size / 3) as usize * 2);
        let mut answer = Vec::with_capacity((size / 3) as usize * 2);
        temp.push((size, true));

        while !temp.is_empty() {
            let t = temp.pop().unwrap();
            if let Some(s) = solve(t) {
                s.into_iter().for_each(|s| {
                    temp.push(s);
                });
            } else {
                answer.push(t);
            }
        }

        answer.into_iter().for_each(|(value, ok)| {
            if ok {
                // (0..value).for_each(|_| write!(writer, "-").unwrap());
                write!(writer, "-").unwrap();
            } else {
                (0..value).for_each(|_| write!(writer, " ").unwrap());
            }
        });

        writeln!(writer).unwrap();
    }

    writer.flush().unwrap();
}

fn solve((value, isbar): (i64, bool)) -> Option<[(i64, bool); 3]> {
    if isbar && value != 1 {
        let temp = value / 3;
        Some([(temp, true), (temp, false), (temp, true)])
    } else {
        None
    }
}
