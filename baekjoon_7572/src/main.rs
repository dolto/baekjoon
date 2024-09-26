use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut res = input.trim().parse::<i32>().unwrap() - 4;
    let mut ten = 0;
    let mut spell = 0;

    if res < 0 {
        // 거꾸로
        res *= -1;
        res %= 60;
        for _ in 0..res {
            ten = if ten <= 0 { 9 } else { ten - 1 };
            spell = if spell <= 0 { 11 } else { spell - 1 };
        }
    } else {
        res %= 60;
        for _ in 0..res {
            ten = (ten + 1) % 10;
            spell = (spell + 1) % 12;
        }
    }

    stdout()
        .write_all(format!("{}{}", (spell as u8 + 'A' as u8) as char, ten).as_bytes())
        .unwrap();
}
