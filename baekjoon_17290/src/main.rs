use std::{
    collections::VecDeque,
    fmt::Display,
    io::{stdin, stdout, BufWriter, Read, Write},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Status {
    DeadZone,
    SafeZone,
    Moved,
}
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::DeadZone => f.write_str("o"),
            _ => f.write_str("x"),
        }
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut table = [[Status::SafeZone; 10]; 10];
    let mut quea = VecDeque::with_capacity(100);

    quea.push_back((
        input.next().unwrap().parse::<usize>().unwrap() - 1,
        input.next().unwrap().parse::<usize>().unwrap() - 1,
        0,
    ));

    for i in 0..10 {
        let mut temp = input.next().unwrap().chars();
        for j in 0..10 {
            let temp = temp.next().unwrap();
            if temp == 'o' {
                for t in 0..10 {
                    if let Some(tile) = table.get_mut(i + t) {
                        tile[j] = Status::DeadZone;
                    }
                    if let Some(tile) = table[i].get_mut(j + t) {
                        *tile = Status::DeadZone;
                    }
                    if i >= t {
                        if let Some(tile) = table.get_mut(i - t) {
                            tile[j] = Status::DeadZone;
                        }
                    }
                    if j >= t {
                        if let Some(tile) = table[i].get_mut(j - t) {
                            *tile = Status::DeadZone;
                        }
                    }
                }
            }
        }
    }
    let mut bw = BufWriter::new(stdout().lock());

    // for i in 0..10 {
    //     for j in 0..10 {
    //         bw.write(table[i][j].to_string().as_bytes()).unwrap();
    //     }
    //     bw.write("\n".as_bytes()).unwrap();
    // }

    while let Some((r, c, answer)) = quea.pop_front() {
        if table[r][c] == Status::SafeZone {
            bw.write(answer.to_string().as_bytes()).unwrap();
            return;
        }

        table[r][c] = Status::Moved;
        if let Some(tr) = r.checked_sub(1) {
            if table[tr][c] != Status::Moved {
                quea.push_back((tr, c, answer + 1));
            }
        }
        if r + 1 < 10 {
            if table[r + 1][c] != Status::Moved {
                quea.push_back((r + 1, c, answer + 1));
            }
        }
        if let Some(tc) = c.checked_sub(1) {
            if table[r][tc] != Status::Moved {
                quea.push_back((r, tc, answer + 1));
            }
        }
        if c + 1 < 10 {
            if table[r][c + 1] != Status::Moved {
                quea.push_back((r, c + 1, answer + 1));
            }
        }
    }
}
