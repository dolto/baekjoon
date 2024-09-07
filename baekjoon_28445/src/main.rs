use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut hashset = HashSet::with_capacity(4);
    while let Some(s) = input.next() {
        hashset.insert(s);
    }

    let mut hashset: Vec<&str> = hashset.into_iter().collect();
    hashset.sort();

    let mut writer = BufWriter::new(stdout().lock());
    for i in 0..hashset.len() {
        for j in 0..hashset.len() {
            writer
                .write(format!("{} {}\n", hashset[i], hashset[j]).as_bytes())
                .unwrap();
        }
    }

    writer.flush().unwrap();
}
