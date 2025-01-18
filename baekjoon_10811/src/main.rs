use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());

    let mut vec = (0..=n).collect::<Vec<usize>>();
    (0..m).for_each(|_| solve(input.next().unwrap(), input.next().unwrap(), &mut vec));

    let mut bw = BufWriter::new(stdout().lock());
    (1..=n).for_each(|i| write!(bw, "{} ", vec[i]).unwrap());

    bw.flush().unwrap();
}

fn solve(mut l: usize, mut r: usize, vec: &mut Vec<usize>) {
    while l < r {
        vec.swap(l, r);
        l += 1;
        r -= 1;
    }
}
