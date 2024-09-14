use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "n" {
        stdout().write_all(format!("Naver D2").as_bytes()).unwrap();
    } else {
        stdout()
            .write_all(format!("Naver Whale").as_bytes())
            .unwrap();
    }
}
