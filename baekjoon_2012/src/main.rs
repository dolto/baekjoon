use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut table = BinaryHeap::with_capacity(input.next().unwrap());

    (0..table.capacity()).for_each(|_| {
        table.push(Reverse(input.next().unwrap()));
    });

    let mut answer = 0;
    (1..=table.capacity()).for_each(|i| {
        let temp = table.pop().unwrap().0 as i64;
        answer += (temp - i as i64).abs();
    });

    println!("{}", answer);
}
