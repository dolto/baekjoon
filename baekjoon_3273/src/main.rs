use std::{
    collections::BTreeSet,
    io::{Read, Write, stdin, stdout},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());
    let n = input.next().unwrap() as usize;
    let vec = input.by_ref().take(n).map(|i| i).collect::<Vec<i32>>();
    let x = input.next().unwrap();
    let mut memory: BTreeSet<i32> = BTreeSet::new();

    let mut answer = 0;

    for v in vec {
        if v >= x {
            continue;
        }
        if let Some(_) = memory.get(&(x - v)) {
            answer += 1;
        } else {
            memory.insert(v);
        }
    }

    writeln!(stdout(), "{}", answer).unwrap();
    stdout().flush().unwrap();
}
