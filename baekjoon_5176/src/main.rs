use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let tcase = input.next().unwrap();
    let mut writer = BufWriter::new(stdout().lock());

    (0..tcase).for_each(|_| {
        let (p, m) = (input.next().unwrap(), input.next().unwrap());
        let mut log = HashSet::with_capacity(m);
        (0..p).for_each(|_| {
            log.insert(input.next().unwrap());
        });

        writeln!(writer, "{}", p - log.len()).unwrap();
    });

    writer.flush().unwrap();
}
