use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let mut bw = BufWriter::new(stdout());
    for _ in 0..input.next().unwrap() {
        let data = input.next().unwrap();
        let mut answer = false;
        for n in 2..=64 as i64 {
            // 11110100001001000000
            let mut nstr = VecDeque::with_capacity(20);
            let mut temp = data;

            while temp > 0 {
                nstr.push_front(temp % n);
                temp /= n;
            }

            answer = true;
            for i in 0..nstr.len() / 2 {
                if nstr[i] != nstr[nstr.len() - i - 1] {
                    answer = false;
                    break;
                }
            }
            if answer {
                writeln!(bw, "1").unwrap();
                break;
            }
        }
        if !answer {
            writeln!(bw, "0").unwrap();
        }
    }

    bw.flush().unwrap();
}
