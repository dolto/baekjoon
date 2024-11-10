use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let test_case = input.next().unwrap().parse().unwrap();

    let mut bw = BufWriter::new(stdout().lock());
    for _ in 0..test_case {
        let mut message = input.next().unwrap().bytes();
        let count = (message.len() as f32).sqrt() as usize;

        let mut answer = vec![Vec::with_capacity(count); count];

        for i in 0..count {
            for _ in 0..count {
                answer[i].push(message.next().unwrap());
            }
        }

        for i in (0..count).rev() {
            for j in 0..count {
                bw.write(&[answer[j][i]]).unwrap();
            }
        }

        bw.write("\n".as_bytes()).unwrap();
    }

    bw.flush().unwrap();
}
