use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i64>());
    let mut heap = BinaryHeap::with_capacity(3);
    while let Some(temp) = input.next() {
        heap.push(Reverse(temp));
    }

    let mut writer = BufWriter::new(stdout().lock());
    while let Some(Reverse(res)) = heap.pop() {
        writer.write_all(format!("{} ", res).as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}
