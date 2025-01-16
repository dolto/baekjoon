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

    (0..list.len()).for_each(|i| solve(&mut list, String::with_capacity(m), i, 0, m, &mut writer));

    writer.flush().unwrap();
}

fn solve(
    arr: &Vec<usize>,
    mut result: String,
    start: usize,
    count: usize,
    max: usize,
    writer: &mut BufWriter<StdoutLock>,
) {
    if count == max {
        writer.write(format!("{}\n", result).as_bytes()).unwrap();
    } else {
        result.push_str(format!("{} ", arr[start]).as_str());
        if count + 1 == max {
            solve(arr, result, start, count + 1, max, writer);
        } else {
            (start..arr.len()).for_each(|i| solve(arr, result.clone(), i, count + 1, max, writer));
        }
    }
}
