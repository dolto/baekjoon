use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    let vec_count: usize = input.next().unwrap().parse().unwrap();
    let mut sum_vec = Vec::with_capacity(vec_count + 1);
    sum_vec.push(0);
    let count = input.next().unwrap().parse().unwrap();

    let mut sum = 0;
    for _ in 0..vec_count {
        let temp = input.next().unwrap().parse::<i64>().unwrap();
        sum += temp;
        sum_vec.push(sum);
    }

    for _ in 0..count {
        let before: usize = input.next().unwrap().parse().unwrap();
        let after: usize = input.next().unwrap().parse().unwrap();

        let res = sum_vec[after] - sum_vec[before - 1];
        writer.write_all(format!("{res}\n").as_bytes()).unwrap();
    }
}
