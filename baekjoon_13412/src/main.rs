use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let tcase = input.next().unwrap();

    let mut bw = BufWriter::new(stdout().lock());
    for _ in 0..tcase {
        let problum = input.next().unwrap();
        let answer = solve(problum);
        writeln!(bw, "{}", answer).unwrap();
    }
    bw.flush().unwrap();
}

fn solve(problum: usize) -> usize {
    let mut i = 1;
    let mut answer = 0;
    while i * i <= problum {
        let temp = problum % i;
        if temp == 0 {
            let a = problum / i;
            let b = i;
            if gcd(a, b) == 1 {
                answer += 1;
            }
        }
        i += 1;
    }
    answer
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
