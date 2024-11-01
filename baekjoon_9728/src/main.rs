use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut bw = BufWriter::new(stdout().lock());
    for i in 1..=input.next().unwrap() {
        let count = input.next().unwrap();
        let target = input.next().unwrap();

        let mut list = HashSet::with_capacity(count);
        let mut answer = 0;
        for _ in 0..count {
            list.insert(input.next().unwrap());
        }

        for &i in list.iter() {
            if i < target {
                if list.contains(&(target - i)) {
                    answer += 1;
                }
            }
        }

        writeln!(bw, "Case #{}: {}", i, answer / 2).unwrap();
    }

    bw.flush().unwrap();
}
