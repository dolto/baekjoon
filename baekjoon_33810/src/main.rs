use std::io::{Write, stdin, stdout};

fn main() {
    let sci = "SciComLove".bytes();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut answer = 0;
    input.bytes().zip(sci).for_each(|(a, b)| {
        if a != b {
            answer += 1;
        }
    });

    write!(stdout(), "{}", answer).unwrap();
    stdout().flush().unwrap();
}
