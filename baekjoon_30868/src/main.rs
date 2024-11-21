use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>())
        .skip(1);

    let mut bw = BufWriter::new(stdout());
    for n in input {
        let plus = n / 5;
        let bar = n % 5;
        for i in 0..plus {
            if i == plus - 1 && bar == 0 {
                bw.write("++++".as_bytes()).unwrap();
            } else {
                bw.write("++++ ".as_bytes()).unwrap();
            }
        }
        for _ in 0..bar {
            bw.write("|".as_bytes()).unwrap();
        }
        bw.write("\n".as_bytes()).unwrap();
    }
    bw.flush().unwrap();
}
