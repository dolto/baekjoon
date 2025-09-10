use std::io::{Read, Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut input = input.lines();
    let mut front: Vec<char> = input.next().unwrap().chars().collect();
    let n = input.next().unwrap().parse::<usize>().unwrap();
    let mut back = Vec::with_capacity(n);

    input.for_each(|s| {
        let mut command = s.chars();
        let c = command.next().unwrap();
        command.next();
        match c {
            'L' => {
                if let Some(s) = front.pop() {
                    back.push(s);
                }
            }
            'D' => {
                if let Some(s) = back.pop() {
                    front.push(s);
                }
            }
            'B' => {
                front.pop();
            }
            'P' => {
                front.push(command.next().unwrap());
            }
            _ => {}
        }
    });

    let mut answer = String::with_capacity(front.len() + back.len());

    for f in front {
        answer.push(f);
    }
    for &b in back.iter().rev() {
        answer.push(b);
    }

    writeln!(stdout(), "{}", answer).unwrap();
    stdout().flush().unwrap();
}
