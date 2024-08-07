use std::io::{stdin, stdout, BufWriter, Read, StdoutLock, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|a| a.parse::<i32>());

    let mut writer = BufWriter::new(stdout().lock());

    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let mut arr = vec![0; n as usize + 1];
    let mut visited = vec![false; n as usize + 1];

    sequence(n, m, 1, 0, &mut arr, &mut visited, &mut writer);
    writer.flush().unwrap();
}

fn sequence(
    n: i32,
    m: i32,
    num: i32,
    cnt: i32,
    arr: &mut Vec<i32>,
    visited: &mut Vec<bool>,
    writer: &mut BufWriter<StdoutLock>,
) {
    if m == cnt {
        for i in 0..m {
            writer
                .write_all(format!("{} ", arr[i as usize]).as_bytes())
                .unwrap();
        }
        writer.write_all("\n".as_bytes()).unwrap();
        return;
    }
    for i in num..=n {
        if !visited[i as usize] {
            visited[i as usize] = true;
            arr[cnt as usize] = i;
            sequence(n, m, i + 1, cnt + 1, arr, visited, writer);
            visited[i as usize] = false;
        }
    }
}
