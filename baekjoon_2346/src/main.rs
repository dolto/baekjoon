use std::{
    collections::VecDeque,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());

    let count: i32 = input.next().unwrap();
    let mut ballouns = VecDeque::with_capacity(count as usize);
    let mut answers = String::new();

    for i in (1..=count).zip(input) {
        ballouns.push_back(i);
    }

    while let Some((i, v)) = ballouns.pop_front() {
        answers.push_str(&(i.to_string() + " "));
        if v > 0 {
            // for _ in 0..(v - 1) {
            //     if let Some(temp) = ballouns.pop_front() {
            //         ballouns.push_back(temp);
            //     }
            // }
            if !ballouns.is_empty() {
                ballouns.rotate_left((v - 1) as usize % ballouns.len());
            }
        } else {
            // for _ in v..0 {
            //     if let Some(temp) = ballouns.pop_back() {
            //         ballouns.push_front(temp);
            //     }
            // }
            if !ballouns.is_empty() {
                ballouns.rotate_right((-v) as usize % ballouns.len());
            }
        }
    }
    stdout().write_all(answers.as_bytes()).unwrap();
}
