use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let len = input.next().unwrap();
    let count = input.next().unwrap();

    let mut vec = Vec::with_capacity(len);
    let mut res = Vec::with_capacity(len.pow(2));
    while let Some(v) = input.next() {
        vec.push(v);
    }
    vec.sort();

    solve(vec, &mut res, count, Vec::with_capacity(count));

    let mut writer = BufWriter::new(stdout().lock());
    for vec in res.iter() {
        for v in vec.iter() {
            writer.write((v.to_string() + " ").as_bytes()).unwrap();
        }
        writer.write("\n".as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}

fn solve(vec: Vec<usize>, res: &mut Vec<Vec<usize>>, count: usize, out: Vec<usize>) {
    if count == 0 {
        res.push(out);
    } else {
        for vi in 0..vec.len() {
            let mut temp = out.clone();
            temp.push(vec[vi]);
            let mut vec = vec.clone();
            vec.remove(vi);
            solve(vec, res, count - 1, temp);
        }
    }
}
