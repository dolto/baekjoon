use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");
    let count = input.next().unwrap().parse().unwrap();
    let mut writer = BufWriter::new(stdout().lock());

    for _ in 0..count {
        let mut k_line = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .flat_map(|s| s.parse::<i32>());
        let need = k_line.next().unwrap();
        let needs_line = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<i32>());
        let mut total = 0;
        for n in needs_line {
            total += n / need;
        }
        let mut output = total.to_string();
        output.push_str("\n");
        writer.write_all(output.as_bytes()).unwrap();
    }
}
