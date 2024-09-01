use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let count = input.next().unwrap().parse::<usize>().unwrap();

    let mut canvas = Vec::with_capacity(count);

    for _ in 0..count {
        let mut temp = Vec::with_capacity(5);
        for _ in 0..5 {
            temp.push(input.next().unwrap().chars().collect::<Vec<char>>());
        }
        canvas.push(temp);
    }

    let mut lr_min = (0, 0, i32::MAX);
    for i in 0..count {
        for j in (i + 1)..count {
            let mut defrient = 0;
            for k in 0..5 {
                for y in 0..7 {
                    if canvas[i][k][y] != canvas[j][k][y] {
                        defrient += 1;
                    }
                }
            }
            if lr_min.2 > defrient {
                lr_min = (i, j, defrient);
            }
        }
    }

    stdout()
        .write_all(format!("{} {}", lr_min.0 + 1, lr_min.1 + 1).as_bytes())
        .unwrap();
}
