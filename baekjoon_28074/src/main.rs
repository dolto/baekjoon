use std::io::{Write, stdin, stdout};

fn main() {
    const MOBIS: [u8; 5] = [b'M', b'O', b'B', b'I', b'S'];
    let mut find = [false, false, false, false, false];
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.bytes().for_each(|b| {
        MOBIS.iter().enumerate().for_each(|(i, a)| {
            if *a == b {
                find[i] = true;
            }
        });
    });

    if find.iter().all(|b| *b) {
        stdout().write("YES".as_bytes()).unwrap();
    } else {
        stdout().write("NO".as_bytes()).unwrap();
    }
}
