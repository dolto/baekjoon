use std::{
    cmp::Ordering,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");

    let count = input.next().unwrap().parse().unwrap();

    let mut data = Vec::with_capacity(count);

    for _ in 0..count {
        data.push(input.next().unwrap().to_string());
    }

    data.sort_by(|a, b| {
        let mut atemp = a.split_ascii_whitespace();
        let (ato, ater) = (atemp.next().unwrap(), atemp.next().unwrap());
        let mut btemp = b.split_ascii_whitespace();
        let (bto, bter) = (btemp.next().unwrap(), btemp.next().unwrap());

        match ato.cmp(bto) {
            Ordering::Equal => ater.cmp(bter).reverse(),
            t => t,
        }
    });

    let mut bw = BufWriter::new(stdout().lock());

    for s in &data {
        writeln!(bw, "{}", s).unwrap();
    }

    bw.flush().unwrap();
}
