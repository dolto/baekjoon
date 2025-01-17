use std::io::{stdin, stdout, BufWriter, Read, StdoutLock, Write};

fn main() {
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let (n, m) = (input.next().unwrap(), input.next().unwrap());

    let mut list = Vec::with_capacity(n);
    (0..n).for_each(|_| list.push(input.next().unwrap()));
    list.sort();
    dfs(0, String::new(), 0, &mut list, &m, &mut writer);
    writer.flush().unwrap();
}

fn dfs(
    count: usize,
    mut result: String,
    index: usize,
    list: &mut Vec<usize>,
    size: &usize,
    writer: &mut BufWriter<StdoutLock>,
) {
    if index == list.len() {
        return;
    }
    if count == *size {
        result.push('\n');
        writer.write(result.as_bytes()).unwrap();
        return;
    }

    let temp = result.clone();
    result.push_str(format!("{} ", list[index]).as_str());
    dfs(count + 1, result, index, list, size, writer);
    dfs(count, temp, index + 1, list, size, writer);
}
