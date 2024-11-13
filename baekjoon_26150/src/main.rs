use std::io::{stdin, stdout, BufWriter, Read, Write};

#[derive(PartialEq, Eq)]
struct Problume {
    value: Vec<char>,
    d: usize,
    i: usize,
}
impl PartialOrd for Problume {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Problume {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.i.cmp(&other.i)
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let size = input.next().unwrap().parse().unwrap();
    let mut problumes = Vec::with_capacity(size);

    for _ in 0..size {
        let value = input
            .next()
            .unwrap()
            .to_string()
            .to_uppercase()
            .chars()
            .collect();
        let i = input.next().unwrap().parse().unwrap();
        let d = input.next().unwrap().parse().unwrap();
        problumes.push(Problume { value, d, i });
    }

    problumes.sort();

    let mut bw = BufWriter::new(stdout().lock());
    for p in problumes {
        write!(bw, "{}", p.value[p.d - 1]).unwrap();
    }

    bw.flush().unwrap();
}
