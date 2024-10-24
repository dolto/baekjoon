use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split('\n');

    let first = input.next().unwrap().chars().collect::<Vec<char>>();
    let last = input.next().unwrap().chars().collect::<Vec<char>>();

    let mut answer = 0;
    for f in 0..first.len() {
        let mut temp = 0;
        for &l in last.iter() {
            if let Some(&fl) = first.get(f + temp) {
                if fl == l {
                    temp += 1;
                } else {
                    temp = 0;
                }
            }
        }
        answer = answer.max(temp);
    }

    stdout().write_all(answer.to_string().as_bytes()).unwrap();
}
