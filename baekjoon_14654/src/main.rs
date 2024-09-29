use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let count = input.next().unwrap() as usize;

    let mut team1 = vec![0; count];
    let mut team2 = vec![0; count];

    for i in 0..count {
        team1[i] = input.next().unwrap();
    }
    for i in 0..count {
        team2[i] = input.next().unwrap();
    }

    let mut max_win = 1;
    let mut win = 1;

    // true = team2 우선 false = team1 우선
    let mut frist = is_win(team1[0], team2[0]);

    for i in 1..count {
        if frist {
            if is_win(team1[i], team2[i]) {
                win += 1;
                max_win = win.max(max_win);
            } else {
                frist = false;
                win = 1;
            }
        } else {
            if is_win(team2[i], team1[i]) {
                win += 1;
                max_win = win.max(max_win);
            } else {
                frist = true;
                win = 1;
            }
        }
    }

    stdout().write_all(max_win.to_string().as_bytes()).unwrap();
}

fn is_win(s: i32, o: i32) -> bool {
    match s {
        1 => {
            if o == 3 {
                true
            } else {
                false
            }
        }
        2 => {
            if o == 1 {
                true
            } else {
                false
            }
        }
        3 => {
            if o == 2 {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
