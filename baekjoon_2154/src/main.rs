use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut res = String::new();
    for i in 1..=input {
        res.push_str(i.to_string().as_str());
    }
    let index = res.split(input.to_string().as_str()).next().unwrap().len() + 1;

    stdout().write_all(format!("{index}").as_bytes()).unwrap();
}
