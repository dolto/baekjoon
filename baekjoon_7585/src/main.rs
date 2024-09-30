use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split("\n");

    let mut stack = Vec::with_capacity(250);
    let mut legal;

    let mut writer = BufWriter::new(stdout().lock());
    while let Some(chs) = input.next() {
        if chs == "#" {
            break;
        }
        legal = true;
        stack.clear();
        let chs = chs.chars();

        for c in chs {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        legal = false;
                        break;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        legal = false;
                        break;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        legal = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if legal && stack.is_empty() {
            writer.write_all("Legal\n".as_bytes()).unwrap();
        } else {
            writer.write_all("Illegal\n".as_bytes()).unwrap();
        }
    }
    writer.flush().unwrap();
}
