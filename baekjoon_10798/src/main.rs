use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::with_capacity(26 * 4 + 10 * 2);
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split('\n');

    let mut list = Vec::with_capacity(26 * 4 + 10 * 2);
    while let Some(s) = input.next() {
        let chs = s.bytes();
        let mut temp = Vec::with_capacity(15);
        for b in chs {
            temp.push(b);
        }

        list.push(temp);
    }

    let mut bw = BufWriter::new(stdout().lock());
    for i in 0..15 {
        for bs in list.iter() {
            if let Some(&b) = bs.get(i) {
                bw.write(&[b]).unwrap();
            }
        }
    }

    bw.flush().unwrap();
}
