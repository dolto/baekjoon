use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let total: i32 = input
        .trim()
        .chars()
        .map(|c| {
            let pars = c as i32;
            if pars < 97 {
                // 소문자인 경우
                pars - 38
            } else {
                pars - 96
            }
        })
        .sum();

    if prime_word(total) {
        stdout()
            .write_all("It is a prime word.".as_bytes())
            .unwrap();
    } else {
        stdout()
            .write_all("It is not a prime word.".as_bytes())
            .unwrap();
    }
}

fn prime_word(total: i32) -> bool {
    for i in 2..total.max(2) {
        if total % i == 0 {
            return false;
        }
    }
    true
}
