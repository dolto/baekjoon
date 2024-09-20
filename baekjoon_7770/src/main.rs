use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<i64>().unwrap();

    let mut floor: i64 = 1;
    loop {
        let need = floor.pow(2) + (floor - 1).pow(2);
        if input >= need {
            floor += 1;
            input -= need
        } else {
            floor -= 1;
            break;
        }
    }
    stdout().write_all(floor.to_string().as_bytes()).unwrap();
}
