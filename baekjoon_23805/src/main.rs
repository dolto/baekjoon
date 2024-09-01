use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut output = String::new();

    for _ in 0..input {
        for _ in 0..input {
            output.push_str("@@@");
        }
        for _ in 0..input {
            output.push_str(" ");
        }
        for _ in 0..input {
            output.push_str("@");
        }
        output.push_str("\n");
    }
    for _ in 0..input * 3 {
        for _ in 0..input {
            output.push_str("@");
        }
        for _ in 0..input {
            output.push_str(" ");
        }
        for _ in 0..input {
            output.push_str("@");
        }
        for _ in 0..input {
            output.push_str(" ");
        }
        for _ in 0..input {
            output.push_str("@");
        }
        output.push_str("\n");
    }
    for _ in 0..input {
        for _ in 0..input {
            output.push_str("@");
        }
        for _ in 0..input {
            output.push_str(" ");
        }
        for _ in 0..input {
            output.push_str("@@@");
        }
        output.push_str("\n");
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
