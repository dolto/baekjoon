use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    io::{stdin, stdout, Read, Write},
};

#[derive(PartialEq, Eq)]
struct Meeting(i32, i32);
impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.1 > other.1 {
            Some(Ordering::Greater)
        } else if self.1 < other.1 {
            Some(Ordering::Less)
        } else {
            if self.0 > other.0 {
                Some(Ordering::Greater)
            } else if self.0 < other.0 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}
impl Ord for Meeting {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 > other.1 {
            Ordering::Greater
        } else if self.1 < other.1 {
            Ordering::Less
        } else {
            if self.0 > other.0 {
                Ordering::Greater
            } else if self.0 < other.0 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let count = input.next().unwrap() as usize;
    let mut heap = BinaryHeap::with_capacity(count);

    for _ in 0..count {
        let left = input.next().unwrap();
        let right = input.next().unwrap();
        heap.push(Reverse(Meeting(left, right)));
    }

    // let mut vec = Vec::with_capacity(count);
    // let mut min = heap.peek().unwrap().0 .0;
    let mut max = heap.pop().unwrap().0 .1;
    let mut result = 1;
    while let Some(Reverse(meeting)) = heap.pop() {
        if max <= meeting.0 {
            max = meeting.1;
            result += 1;
        }
    }

    stdout().write_all(format!("{result}").as_bytes()).unwrap();
}
