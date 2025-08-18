use std::io::{Read, stdin};
fn main() {
    let mut input = stdin();
    let mut buffer = String::new();
    input.read_line(&mut String::new()).unwrap();
    input.read_to_string(&mut buffer).unwrap();
    let input = buffer.chars();

    let mut vec = Vec::new();

    for c in input {
        match c {
            '4' | '5' => {
                let len = vec.len();
                if vec.len() < 2 || vec[len - 1] != 'S' || vec[len - 2] != 'P' {
                    vec.push(c);
                }
            }
            _ => vec.push(c),
        }
    }

    let output: String = vec.into_iter().collect();
    print!("{}", output);
}
