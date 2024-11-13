use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");
    let mut bw = BufWriter::new(stdout().lock());

    loop {
        let temp = input.next().unwrap();
        if temp == "*" {
            break;
        }

        let mut set = HashSet::with_capacity(temp.len());
        let temp = temp.chars();

        for c in temp {
            set.insert(c);
        }

        let mut answer = "Y";
        for c in 'a'..='z' {
            if !set.contains(&c) {
                answer = "N";
                break;
            }
        }

        writeln!(bw, "{answer}").unwrap();
    }

    bw.flush().unwrap();
}
