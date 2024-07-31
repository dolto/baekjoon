use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    let count = input.next().unwrap().parse().unwrap();
    for i in 1..=count {
        let a: i64 = input.next().unwrap().parse().unwrap();
        let b: i64 = input.next().unwrap().parse().unwrap();
        let c: i64 = input.next().unwrap().parse().unwrap();

        let max = a.max(b.max(c));
        let other = a + b + c - max;
        if a == b && a == c {
            writer
                .write_all(format!("Case #{}: equilateral\n", i).as_bytes())
                .unwrap();
        } else if max >= other {
            writer
                .write_all(format!("Case #{}: invalid!\n", i).as_bytes())
                .unwrap();
        } else if a == b || a == c || b == c {
            writer
                .write_all(format!("Case #{}: isosceles\n", i).as_bytes())
                .unwrap();
        } else {
            writer
                .write_all(format!("Case #{}: scalene\n", i).as_bytes())
                .unwrap();
        }
    }

    writer.flush().unwrap();
}
