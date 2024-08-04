use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let trees_count = input.next().unwrap().parse().unwrap();
    let needs: i64 = input.next().unwrap().parse().unwrap();

    let mut trees = Vec::with_capacity(trees_count);
    for _ in 0..trees_count {
        let temp = input.next().unwrap().parse::<i64>().unwrap();
        trees.push(temp);
    }

    trees.sort();

    let mut get;
    let mut max = trees[trees_count - 1];
    let mut min = 0;
    let mut cut = (max + min) / 2;

    // println!("cut: {cut}");
    while min < max - 1 {
        // println!("min: {min}\nmax: {max}");
        get = 0;
        for t in trees.iter() {
            get += (t - cut).max(0);
        }
        if get >= needs {
            min = cut;
        } else {
            max = cut;
        }
        cut = (max + min) / 2;
    }
    stdout()
        .write_all(format!("{}", max - 1).as_bytes())
        .unwrap();
}
