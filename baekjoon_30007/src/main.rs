use std::io::{stdin, stdout, BufReader, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();
    let count = input.next().unwrap().trim().parse().unwrap();

    let mut output = String::new();

    for _ in 0..count {
        let nuddle_coefficient: i32 = input.next().unwrap().trim().parse().unwrap();
        let base_water: i32 = input.next().unwrap().trim().parse().unwrap();
        let nuddle_count: i32 = input.next().unwrap().trim().parse().unwrap();

        let result = nuddle_coefficient * (nuddle_count - 1) + base_water;
        output.push_str(format!("{result}\n").as_str());
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
