use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let n = input.next().unwrap() as usize;
    let m = input.next().unwrap() as usize;

    let mut ns = vec![Vec::with_capacity(m); n];
    let mut answer = vec![(false, false); n];
    for ms in ns.iter_mut() {
        for _ in 0..m {
            ms.push(input.next().unwrap());
        }
    }
    if ns.len() == 1 {
        stdout().write("YES".as_bytes()).unwrap();
        return;
    }

    let maker = ns.pop().unwrap();
    for i in 0..m {
        for j in 0..n - 1 {
            if maker[i] != ns[j][i] {
                if maker[i] == 0 {
                    // 기준이 못들었는데 내가 들음
                    answer[j].1 = true;
                } else {
                    // 기준이 못들었는데 내가 못들음
                    answer[j].0 = true;
                }

                if answer[j].0 && answer[j].1 {
                    // 두 경우가 모두 공존하면 불가능하다는 의미
                    stdout().write("NO".as_bytes()).unwrap();
                    return;
                }
            }
        }
    }
    stdout().write("YES".as_bytes()).unwrap();

    // 다른사람이 들었는데 못들었다와 다른사람이 못들었는데 들었다가 공존하면 안됨 + 그 반대도 안됨
}
