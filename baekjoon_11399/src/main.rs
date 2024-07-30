use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufReader, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let count = input.next().unwrap().trim().parse().unwrap();
    let mut min_heap = BinaryHeap::with_capacity(count);

    for _ in 0..count {
        let res: i32 = input.next().unwrap().trim().parse().unwrap();
        min_heap.push(Reverse(res));
    }

    let mut some = 0;
    let mut vec = Vec::with_capacity(count);
    while let Some(Reverse(min)) = min_heap.pop() {
        some += min;
        vec.push(some);
    }
    some = 0;
    for s in vec.iter() {
        some += s;
    }
    writer.write_all(format!("{some}").as_bytes()).unwrap();
    writer.flush().unwrap();
}
