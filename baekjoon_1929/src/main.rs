use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let offset = input.next().unwrap();
    let maxindex = input.next().unwrap();

    let mut eratores = vec![true; maxindex];

    eratores[0] = false;
    for i in 1..maxindex {
        if eratores[i] {
            for j in (i..maxindex).step_by(i + 1).skip(1) {
                eratores[j] = false;
            }
        }
    }

    let eratores = eratores
        .into_iter()
        .enumerate()
        .filter(|b| b.1 && (offset..=maxindex).contains(&(b.0 + 1)))
        .map(|b| b.0 + 1);
    let mut bw = BufWriter::new(stdout().lock());

    // writeln!(bw, "{:?}", eratores.collect::<Vec<usize>>()).unwrap();
    for b in eratores {
        writeln!(bw, "{}", b).unwrap();
    }

    bw.flush().unwrap();
}
