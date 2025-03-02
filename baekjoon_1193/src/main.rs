use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<i32>().unwrap();

    let mut line = 1;
    while line < input {
        input -= line;
        line += 1;
    }

    let (x, y);
    if line % 2 == 0 {
        (y, x) = (input, line - input + 1);
    } else {
        (y, x) = (line - input + 1, input);
    }

    write!(stdout(), "{y}/{x}").unwrap();
    stdout().flush().unwrap();
}
