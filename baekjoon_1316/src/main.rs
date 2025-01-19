use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::with_capacity(3 + 100 * 101);
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut bw = BufWriter::new(stdout().lock());

    let mut answer = input.next().unwrap().parse::<usize>().unwrap();
    for s in input {
        let s = s.bytes();
        let mut hashset = HashSet::with_capacity(26);
        let mut last = 0;

        for b in s {
            if !hashset.insert(b) && last != b {
                answer -= 1;
                break;
            } else {
                last = b;
            }
        }
    }

    bw.write(answer.to_string().as_bytes()).unwrap();
    bw.flush().unwrap();
}
