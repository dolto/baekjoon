use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let prime_before = input.next().unwrap();
    let prime_after = (input.next().unwrap().to_string() + prime_before)
        .parse::<i64>()
        .unwrap();

    let prime_before = prime_before.parse::<i64>().unwrap();

    if is_prime(prime_before) && is_prime(prime_after) {
        writeln!(stdout(), "Yes").unwrap();
    } else {
        writeln!(stdout(), "No").unwrap();
    }
    stdout().flush().unwrap();
}

fn is_prime(prime: i64) -> bool {
    let mut count = 2;
    let mut answer = true;
    while count * count <= prime {
        if prime % count == 0 {
            answer = false;
            break;
        }
        count += 1;
    }

    answer
}
