use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

#[derive(PartialEq, Eq)]
struct AbsElement(i32);
impl PartialOrd for AbsElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sabs = self.0.abs();
        let oabs = other.0.abs();

        if sabs > oabs {
            Some(Ordering::Greater)
        } else if sabs < oabs {
            Some(Ordering::Less)
        } else if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl Ord for AbsElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sabs = self.0.abs();
        let oabs = other.0.abs();

        if sabs > oabs {
            Ordering::Greater
        } else if sabs < oabs {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else if self.0 < other.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut input = String::new();
    let mut output = String::new();

    reader.read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    // Rust std::collection에는 BinaryHeap이라는 기본적인 최대힙이 존재한다.
    let mut hip: BinaryHeap<Reverse<AbsElement>> = BinaryHeap::with_capacity(count);

    for _ in 0..count {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let res = input.trim().parse::<i32>().unwrap();

        if res == 0 {
            // Reverse로 감싼값은 비교연산이 반대로 돌아간다.
            if let Some(Reverse(max)) = hip.pop() {
                output += format!("{}\n", max.0).as_str();
            } else {
                output += "0\n";
            }
        } else {
            hip.push(Reverse(AbsElement(res)));
        }
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
