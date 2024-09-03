use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut nodes = vec![HashSet::new(); input.next().unwrap()];
    let count = input.next().unwrap();
    let mut hashset = HashSet::with_capacity(count * 2);
    let start = input.next().unwrap() - 1;

    let mut writer = BufWriter::new(stdout().lock());
    for _ in 0..count {
        let left = input.next().unwrap() - 1;
        let right = input.next().unwrap() - 1;

        nodes[left].insert(right);
        nodes[right].insert(left);
    }

    // dfs
    let mut nexts = VecDeque::with_capacity(count * 2);
    nexts.push_back(start);
    while let Some(n) = nexts.pop_back() {
        if !hashset.contains(&n) {
            hashset.insert(n);
            writer.write(format!("{} ", n + 1).as_bytes()).unwrap();
            let mut temp = nodes[n].iter().collect::<Vec<&usize>>();
            temp.sort();
            while let Some(on) = temp.pop() {
                nexts.push_back(*on);
            }
        }
    }
    writer.write("\n".as_bytes()).unwrap();

    // bfs
    nexts.push_back(start);
    hashset.clear();

    while let Some(n) = nexts.pop_back() {
        if !hashset.contains(&n) {
            hashset.insert(n);
            writer.write(format!("{} ", n + 1).as_bytes()).unwrap();
            let mut temp = nodes[n].iter().collect::<Vec<&usize>>();
            temp.sort();
            temp = temp.iter().rev().map(|u| *u).collect();
            println!("{:?}", temp);
            while let Some(on) = temp.pop() {
                nexts.push_front(*on);
            }
        }
    }
    writer.flush().unwrap();
}
