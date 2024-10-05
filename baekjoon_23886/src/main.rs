use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input
        .trim()
        .chars()
        .flat_map(|c| c.to_string().parse::<i32>());

    let mut m = input.next().unwrap();
    let mut present = input.next().unwrap();
    m = m - present;
    if m >= 0 {
        stdout().write("NON ALPSOO".as_bytes()).unwrap();
        return;
    }
    let mut up = true;

    let mut alpsoo = true;

    while let Some(n) = input.next() {
        let tm = present - n;
        present = n;
        if tm < 0 {
            if up {
                if tm != m {
                    alpsoo = false;
                    break;
                }
            } else {
                m = tm;
                up = !up;
            }
        } else if tm > 0 {
            if up {
                m = tm;
                up = !up;
            } else {
                if tm != m {
                    alpsoo = false;
                    break;
                }
            }
        } else {
            alpsoo = false;
            break;
        }
    }
    if alpsoo && !up {
        stdout().write("ALPSOO".as_bytes()).unwrap();
    } else {
        stdout().write("NON ALPSOO".as_bytes()).unwrap();
    }
}
