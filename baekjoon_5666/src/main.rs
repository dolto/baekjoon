use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<f32>());
    let mut writer = BufWriter::new(stdout().lock());

    while let (Some(hotdog), Some(poeple)) = (input.next(), input.next()) {
        let res = (hotdog / poeple * 100.).round() / 100.;
        writer.write(format!("{:.2}\n", res).as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}
