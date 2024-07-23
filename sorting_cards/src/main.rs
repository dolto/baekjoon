use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());

    reader.read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut heap = BinaryHeap::with_capacity(count);
    for _ in 0..count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let res = input.trim().parse::<i32>().unwrap();

        heap.push(Reverse(res));
    }

    let mut max = 0;
    while let Some(card) = heap.pop() {
        if let Some(other) = heap.pop() {
            let temp = card.0 + other.0;
            heap.push(Reverse(temp));
            max += temp;
        }
    }
    stdout().write_all(format!("{max}").as_bytes()).unwrap();
}
