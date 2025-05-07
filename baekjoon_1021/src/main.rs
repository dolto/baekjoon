use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let targets: Vec<usize> = input.take(m).collect();
    let mut queue: VecDeque<usize> = (1..=n).collect();

    let mut total_ops = 0;

    for &target in &targets {
        let idx = queue.iter().position(|&x| x == target).unwrap();

        if idx <= queue.len() / 2 {
            for _ in 0..idx {
                let front = queue.pop_front().unwrap();
                queue.push_back(front);
                total_ops += 1;
            }
        } else {
            for _ in 0..(queue.len() - idx) {
                let back = queue.pop_back().unwrap();
                queue.push_front(back);
                total_ops += 1;
            }
        }

        // 제거
        queue.pop_front();
    }

    println!("{}", total_ops);
}
