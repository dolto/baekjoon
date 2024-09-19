use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();
    let mut log = Vec::with_capacity(input);

    log.push(0);
    log.push(1);
    for i in 2..=input {
        let mut count = 1 as usize;
        let mut min = i32::MAX;
        loop {
            let c = count.pow(2);
            if c > i {
                break;
            }
            if min > log[i - c] {
                min = log[i - c];
            }
            count += 1;
        }
        log.push(min + 1);
    }

    stdout()
        .write_all(format!("{}", log[input]).as_bytes())
        .unwrap();
}
