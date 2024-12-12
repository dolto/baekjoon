use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (mut x, mut y) = (input.next().unwrap(), input.next().unwrap());
    let k = input.next().unwrap();
    let mut temp = 0;
    let mut answer = (1, 0);
    let mut can_seat = false;

    // y, x-1, y-1, x-2, y-2, x-3, y-3

    y += 1;

    loop {
        y -= 1;
        if y == 0 {
            break;
        }
        temp += y;
        answer.1 += y;
        if k <= temp {
            answer.1 -= temp - k;
            can_seat = true;
            break;
        }

        x -= 1;
        if x == 0 {
            break;
        }
        temp += x;
        answer.0 += x;
        if k <= temp {
            answer.0 -= temp - k;
            can_seat = true;
            break;
        }

        y -= 1;
        if y == 0 {
            break;
        }
        temp += y;
        answer.1 -= y;
        if k <= temp {
            answer.1 += temp - k;
            can_seat = true;
            break;
        }

        x -= 1;
        if x == 0 {
            break;
        }
        temp += x;
        answer.0 -= x;
        if k <= temp {
            answer.0 += temp - k;
            can_seat = true;
            break;
        }
    }

    if can_seat {
        writeln!(stdout(), "{} {}", answer.0, answer.1).unwrap();
    } else {
        writeln!(stdout(), "0").unwrap();
    }
    stdout().flush().unwrap();
}
