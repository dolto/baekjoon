use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());

    let mut in_people = HashSet::with_capacity(input.next().unwrap());
    let mut answer = 0;

    while let (Some(index), Some(value)) = (input.next(), input.next()) {
        if value == 0 {
            if !in_people.remove(&index) {
                answer += 1;
            }
        } else {
            if !in_people.insert(index) {
                answer += 1;
            }
        }
    }

    answer += in_people
        .stdout()
        .write_all(answer.to_string().as_bytes())
        .unwrap();
}
