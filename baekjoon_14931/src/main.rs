use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i64>());

    let l = input.next().unwrap() as usize;
    let mut river = Vec::with_capacity(l);

    (0..l).for_each(|_| {
        river.push(input.next().unwrap());
    });

    let mut danswer = 0;
    let mut panswer = 0;

    for d in 1..=l {
        let start = d - 1;
        let mut ptemp = 0;
        for i in (start..l).step_by(d) {
            ptemp += river[i];
        }

        if ptemp > panswer {
            danswer = d;
            panswer = ptemp;
        }
    }

    if panswer < 0 {
        danswer = 0;
        panswer = 0;
    }

    stdout()
        .write(format!("{} {}", danswer, panswer).as_bytes())
        .unwrap();
}
