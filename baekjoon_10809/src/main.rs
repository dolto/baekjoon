use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
};

fn main() {
    let mut input = String::with_capacity(101);
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().bytes();

    let mut hashmap = HashMap::with_capacity(100);

    for (i, b) in input.enumerate() {
        hashmap.entry(b).or_insert(i as i32);
    }

    let mut bw = BufWriter::with_capacity(26, stdout().lock());
    ('a' as u8..='z' as u8)
        .for_each(|u| write!(bw, "{} ", *hashmap.entry(u).or_insert(-1)).unwrap());

    bw.flush().unwrap();
}
