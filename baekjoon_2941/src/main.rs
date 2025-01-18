use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(101);
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().bytes().peekable();
    let mut answer = 0;

    // &str.strip_prefix(arg: &str) -> Option<&str> 을 이용하면 맨 앞부분이 arg와 일치하면 그부분을 제거한다
    // &str.strip_suffix(arg: &str) -> Option<&str> 을 이용하면 맨 뒷부분이 arg와 일치하면 그부분을 제거한다

    while let Some(b) = input.next() {
        answer += 1;
        match b {
            b'c' => {
                if let Some(&b) = input.peek() {
                    if b == b'=' || b == b'-' {
                        answer -= 1;
                    }
                }
            }
            b'd' => {
                if let Some(&b) = input.peek() {
                    if b == b'-' {
                        answer -= 1;
                    } else if b == b'z' {
                        input.next();
                        answer += 1;
                        if let Some(&b) = input.peek() {
                            if b == b'=' {
                                answer -= 2;
                            }
                        }
                    }
                }
            }
            b's' | b'z' => {
                if let Some(&b) = input.peek() {
                    if b == b'=' {
                        answer -= 1;
                    }
                }
            }
            b'l' | b'n' => {
                if let Some(&b) = input.peek() {
                    if b == b'j' {
                        answer -= 1;
                    }
                }
            }
            _ => {}
        }
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
