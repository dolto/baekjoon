use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::with_capacity(7 + 4 * 100 * 200);
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let mut arr1 = vec![Vec::with_capacity(m); n];
    let mut arr2 = vec![Vec::with_capacity(m); n];

    for i in 0..n {
        for _ in 0..m {
            arr1[i].push(input.next().unwrap());
        }
    }
    for i in 0..n {
        for _ in 0..m {
            arr2[i].push(input.next().unwrap());
        }
    }

    let mut bw = BufWriter::new(stdout().lock());
    for i in 0..n {
        for j in 0..m {
            write!(bw, "{} ", arr1[i][j] + arr2[i][j]).unwrap();
        }
        bw.write("\n".as_bytes()).unwrap();
    }
    bw.flush().unwrap();
}
