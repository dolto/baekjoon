use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut scores = Vec::with_capacity(n);
    let mut max: f32 = 0.;
    for f in input {
        let f = f.parse::<f32>().unwrap();
        max = max.max(f);
        scores.push(f);
    }

    let answer = scores.into_iter().map(|f| f / max * 100.).sum::<f32>() / n as f32;

    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
