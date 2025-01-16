use std::io::{stdin, stdout, Write};

const PLUS: u8 = '+' as u8;
const MINUS: u8 = '-' as u8;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = format!("{}+", input.trim());
    let input = input.bytes();

    let mut temp = Vec::with_capacity(5);
    let mut num = 0;
    let mut is_minus = false;

    for b in input {
        match b {
            PLUS => {
                let n = temp.iter().collect::<String>().parse::<i32>().unwrap();
                temp.clear();
                if is_minus {
                    // println!("-{}", n);
                    num -= n;
                } else {
                    // println!("+{}", n);
                    num += n;
                }
            }
            MINUS => {
                let n = temp.iter().collect::<String>().parse::<i32>().unwrap();
                temp.clear();
                if is_minus {
                    // println!("-{}", n);
                    num -= n;
                } else {
                    // println!("+{}", n);
                    num += n;
                }
                is_minus = true;
            }
            n => {
                let n = n as char;
                // println!("char: {n}");
                temp.push(n);
            }
        }
    }

    stdout().write(num.to_string().as_bytes()).unwrap();
}
