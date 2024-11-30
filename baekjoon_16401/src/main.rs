use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let size = input.next().unwrap();
    let count = input.next().unwrap();
    let mut ls = Vec::with_capacity(size);
    let mut start = 1;
    let mut end = 0;
    let mut answer = 0;

    for _ in 0..count {
        let temp = input.next().unwrap();
        ls.push(temp);
        if end < temp {
            end = temp;
        }
    }

    while start <= end {
        let mid = (start + end) / 2;
        let mut count = 0;
        for &i in ls.iter() {
            count += i / mid;
        }
        if count >= size {
            answer = usize::max(answer, mid);
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    stdout().write(answer.to_string().as_bytes()).unwrap();

    // 6 4
    // 12 10 5 5
    // 6 6 10 5 5
    // 6 6 5 5 5 5

    // 4 3
    // 1 1 15
    // 1 1 7 8
    // 1 1 7 4 4
    // 1 1 3 4 4 4
}
