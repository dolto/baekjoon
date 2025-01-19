use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::with_capacity(58 * 20);
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut total = 0.;
    let mut count = 0.;
    while let Some(_) = input.next() {
        let point = input.next().unwrap().parse::<f32>().unwrap();
        let class = match input.next().unwrap() {
            "A+" => 4.5,
            "A0" => 4.0,
            "B+" => 3.5,
            "B0" => 3.0,
            "C+" => 2.5,
            "C0" => 2.0,
            "D+" => 1.5,
            "D0" => 1.0,
            "P" => 999.,
            _ => 0.0,
        };

        if class < 900. {
            count += point;
            total += point * class;
        }
    }

    stdout()
        .write((total / count).to_string().as_bytes())
        .unwrap();
    stdout().flush().unwrap();
}
