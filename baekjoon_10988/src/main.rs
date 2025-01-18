use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::with_capacity(101);
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().bytes().collect::<Vec<u8>>();

    let (mut l, mut r) = (0_usize, input.len() - 1);

    while l < r {
        if input[l] != input[r] {
            stdout().write("0".as_bytes()).unwrap();
            stdout().flush().unwrap();
            return;
        }
        l += 1;
        r -= 1;
    }
    stdout().write("1".as_bytes()).unwrap();
    stdout().flush().unwrap();
}
