use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let tcase = input.next().unwrap().parse().unwrap();
    let mut bw = BufWriter::new(stdout().lock());
    for _ in 0..tcase {
        let s = input.next().unwrap();
        let p = input.next().unwrap();

        let cv = s.split(p);
        let mut answer = 0;
        for s in cv {
            answer += 1 + s.len();
        }

        answer -= 1;

        writeln!(bw, "{answer}").unwrap();
    }
    bw.flush().unwrap();
}
