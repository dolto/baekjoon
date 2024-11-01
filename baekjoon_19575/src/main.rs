use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let mut answer;
    let count = input.next().unwrap();
    let mut foo = Vec::with_capacity(count as usize + 1);
    let x = input.next().unwrap();
    for _ in 0..=count {
        foo.push((input.next().unwrap(), input.next().unwrap()));
    }
    foo.sort_by(|a, b| b.1.cmp(&a.1));
    answer = foo[0].0;
    for &(a, _) in foo.iter().skip(1) {
        answer = (x * answer + a) % (i64::pow(10, 9) + 7);
    }
    stdout().write(answer.to_string().as_bytes()).unwrap();
}
