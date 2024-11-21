use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let tcase = input.next().unwrap().parse().unwrap();
    let mut bw = BufWriter::new(stdout());

    for _ in 0..tcase {
        let li = input.next().unwrap().parse().unwrap();
        let ci = input.next().unwrap().parse().unwrap();

        let mut map = vec![Vec::with_capacity(ci); li];
        for i in 0..li {
            for b in input.next().unwrap().bytes() {
                map[i].push(b);
            }
        }

        let lm = input.next().unwrap().parse().unwrap();
        let cm = input.next().unwrap().parse().unwrap();

        let mut world = vec![Vec::with_capacity(cm); lm];
        for i in 0..lm {
            for b in input.next().unwrap().bytes() {
                world[i].push(b);
            }
        }

        let mut answer = 0;

        for im in 0..=lm - li {
            for jm in 0..=cm - ci {
                let mut ok = true;
                'ci: for ii in 0..li {
                    for ji in 0..ci {
                        if map[ii][ji] != world[im + ii][jm + ji] {
                            ok = false;
                            break 'ci;
                        }
                    }
                }
                if ok {
                    answer += 1;
                }
            }
        }

        writeln!(bw, "{}", answer).unwrap();
    }
    bw.flush().unwrap();
}
