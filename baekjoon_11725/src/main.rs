use std::{
    // cmp::Ordering,
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

#[derive(Clone, Debug)]
struct Tree {
    parent: usize,
    child: HashSet<i32>,
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let count = input.next().unwrap() as usize;
    let mut tree = vec![
        Tree {
            parent: 0,
            child: HashSet::new()
        };
        count
    ];
    for _ in 1..count {
        let left = input.next().unwrap();
        let right = input.next().unwrap();

        tree[left as usize - 1].child.insert(right);
        tree[right as usize - 1].child.insert(left);
    }
    solve(0, &mut tree);

    let mut writer = BufWriter::new(stdout().lock());

    for i in 1..count {
        write!(writer, "{}\n", tree[i].parent).unwrap();
    }
    writer.flush().unwrap();
}
fn solve(tindex: usize, tree: &mut Vec<Tree>) {
    let temp: Vec<i32> = tree[tindex].child.iter().cloned().collect();
    for i in temp {
        let index = i as usize - 1;
        if tree[index].parent == 0 {
            tree[index].parent = tindex + 1;
            solve(index, tree);
        }
    }
}
