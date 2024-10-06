use std::{
    collections::BinaryHeap,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());
    let mut burger_heap = BinaryHeap::with_capacity(input.next().unwrap());
    let mut side_heap = BinaryHeap::with_capacity(input.next().unwrap());
    let mut juice_heap = BinaryHeap::with_capacity(input.next().unwrap());

    for _ in 0..burger_heap.capacity() {
        burger_heap.push(input.next().unwrap());
    }
    for _ in 0..side_heap.capacity() {
        side_heap.push(input.next().unwrap());
    }
    for _ in 0..juice_heap.capacity() {
        juice_heap.push(input.next().unwrap());
    }

    let mut total = 0;
    let mut total_with_discount = 0;

    while !burger_heap.is_empty() && !side_heap.is_empty() && !juice_heap.is_empty() {
        let mut temp = 0;
        temp += burger_heap.pop().unwrap();
        temp += side_heap.pop().unwrap();
        temp += juice_heap.pop().unwrap();
        total += temp;
        total_with_discount += temp / 10 * 9;
    }
    while let Some(p) = burger_heap.pop() {
        total += p;
        total_with_discount += p;
    }
    while let Some(p) = side_heap.pop() {
        total += p;
        total_with_discount += p;
    }
    while let Some(p) = juice_heap.pop() {
        total += p;
        total_with_discount += p;
    }

    stdout()
        .write(format!("{total}\n{total_with_discount}").as_bytes())
        .unwrap();
}
