use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut students = [true; 31];

    for i in input {
        students[i] = false;
    }

    let mut students = students
        .into_iter()
        .enumerate()
        .filter(|(_, v)| *v)
        .skip(1)
        .map(|(i, _)| i);
    write!(
        stdout(),
        "{}\n{}",
        students.next().unwrap(),
        students.next().unwrap()
    )
    .unwrap();
    stdout().flush().unwrap();
}
