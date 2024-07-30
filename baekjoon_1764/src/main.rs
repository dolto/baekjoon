use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::{stdin, stdout, BufReader, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let heard_count = input.next().unwrap().trim().parse().unwrap();
    let seem_count = input.next().unwrap().trim().parse().unwrap();
    let mut heard = HashSet::with_capacity(heard_count);
    let mut seem_heard = BinaryHeap::with_capacity(heard_count);

    for _ in 0..heard_count {
        let rc = input.next().unwrap().trim();
        heard.insert(rc);
    }

    let mut heard_seems_count = 0;

    for _ in 0..seem_count {
        if let Some(h) = heard.get(input.next().unwrap().trim()) {
            heard_seems_count += 1;
            seem_heard.push(Reverse(h));
        }
    }

    while let Some(Reverse(sh)) = seem_heard.pop() {
        output.push_str(format!("{sh}\n").as_str());
    }

    writer
        .write_all(format!("{heard_seems_count}\n{}", output.trim_end()).as_bytes())
        .unwrap();

    writer.flush().unwrap();
}
