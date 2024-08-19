use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().parse::<i32>().unwrap();

    let mut output = String::new();
    let n = 9;
    while input / n != 0 {
        let p = input % n;
        input = input / n;
        output = format!("{p}{output}");
    }
    output = format!("{input}{output}");

    stdout().write_all(output.as_bytes()).unwrap();
}
