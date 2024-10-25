use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().skip(1);
    let mut br = BufWriter::new(stdout().lock());
    let mut count = 0;

    while let (Some(n), Some(b)) = (input.next(), input.next()) {
        count += 1;
        let n = n.parse().unwrap();
        let mut b = b.parse::<i32>().unwrap();
        let mut answer = 0;

        let mut ai = BinaryHeap::with_capacity(n);
        for _ in 0..n {
            ai.push(Reverse(input.next().unwrap().parse::<i32>().unwrap()));
        }

        while let Some(Reverse(i)) = ai.pop() {
            if b < i {
                break;
            } else {
                answer += 1;
                b -= i;
            }
        }

        writeln!(br, "Case #{count}: {answer}").unwrap();
    }
    br.flush().unwrap();
}
