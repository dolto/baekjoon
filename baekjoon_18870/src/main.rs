use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut output = String::new();

    let count = input.next().unwrap().parse().unwrap();
    let mut hashset = HashSet::with_capacity(count);
    let mut heap = BinaryHeap::with_capacity(count);
    let mut vec = Vec::with_capacity(count);

    for _ in 0..count {
        let x: i64 = input.next().unwrap().parse().unwrap();
        hashset.insert(x);
        vec.push(x);
    }
    for x in hashset.into_iter() {
        heap.push(Reverse(x));
    }
    let mut index_map = HashMap::with_capacity(heap.len());
    let mut count = 0;
    while let Some(Reverse(x)) = heap.pop() {
        index_map.insert(x, count);
        count += 1;
    }
    for x in vec.iter() {
        output.push_str(format!("{} ", index_map.get(x).unwrap()).as_str());
    }

    stdout().write_all(output.trim().as_bytes()).unwrap();
}
