use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let data = input.next().unwrap().to_string();
    let index = input.next().unwrap().parse::<usize>().unwrap();
    stdout()
        .write(format!("{}", &data[(index - 1)..index]).as_bytes())
        .unwrap();
}
