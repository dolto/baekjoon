use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let (c1, c2) = ([3, 2, 4, 2], [4, 1, 3, 1]);

    let mut bw = BufWriter::new(stdout().lock());
    if n == 1 {
        if m == 1 {
            write!(bw, "1\n1").unwrap();
        } else {
            writeln!(bw, "2").unwrap();
            let candidate = [1, 2];
            for i in 0..m - 1 {
                write!(bw, "{} ", candidate[i % 2]).unwrap();
            }
            write!(bw, "{}", candidate[(m - 1) % 2]).unwrap();
        }
    } else {
        let candidate = [1, 2];
        if m == 1 {
            writeln!(bw, "2").unwrap();
            for i in 0..n {
                writeln!(bw, "{}", candidate[i % 2]).unwrap();
            }
        } else {
            writeln!(bw, "4").unwrap();
            for i in 0..n {
                if i % 2 == 0 {
                    for j in 0..m - 1 {
                        write!(bw, "{} ", c1[j % 4]).unwrap();
                    }
                    writeln!(bw, "{}", c1[(m - 1) % 4]).unwrap();
                } else {
                    for j in 0..m - 1 {
                        write!(bw, "{} ", c2[j % 4]).unwrap();
                    }
                    writeln!(bw, "{}", c2[(m - 1) % 4]).unwrap();
                }
            }
        }
    }
    bw.flush().unwrap();
}
