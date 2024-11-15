use std::io::{stdin, stdout, Read, Write};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut answer = 1;
    let mut max = 0;
    let prime_table = make_prime_table(20000);

    for _ in 0..input.next().unwrap() {
        let tanswer = input.next().unwrap();
        let mut temp = tanswer;
        for &p in prime_table.iter() {
            while temp % p == 0 {
                temp /= p;
                if max < p {
                    answer = tanswer;
                    max = p;
                } else if max == p && answer < tanswer {
                    answer = tanswer;
                }
            }
        }
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
}

fn make_prime_table(number: usize) -> Vec<usize> {
    let mut temp = vec![true; number];

    temp[0] = false;
    for i in 1..number {
        let step = i + 1;
        if temp[i] {
            for j in (i + step..number).step_by(step) {
                temp[j] = false;
            }
        }
    }

    let result = temp
        .into_iter()
        .enumerate()
        .filter(|&(_, v)| v)
        .map(|(i, _)| i + 1)
        .collect();

    result
}
