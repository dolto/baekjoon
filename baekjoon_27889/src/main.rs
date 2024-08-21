use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    match input.to_uppercase().trim() {
        "NLCS" => {
            stdout()
                .write_all("North London Collegiate School".as_bytes())
                .unwrap();
        }
        "BHA" => {
            stdout()
                .write_all("Branksome Hall Asia".as_bytes())
                .unwrap();
        }
        "KIS" => {
            stdout()
                .write_all("Korea International School".as_bytes())
                .unwrap();
        }
        "SJA" => {
            stdout()
                .write_all("St. Johnsbury Academy".as_bytes())
                .unwrap();
        }
        _ => {}
    }
}
