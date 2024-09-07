use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};

// enum R {
//     Eight(i32),
//     SixTeen(i32),
//     Ten(i32),
// }
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input_char: VecDeque<char> = input.trim().chars().collect();
    if input_char[0] != '0' && input_char[1] != 'x' {
        stdout().write_all(format!("{input}").as_bytes()).unwrap();
    } else if input_char[0] == '0' && input_char[1] == 'x' {
        // 16진수
        let mut res = 0;
        let mut count = 0;
        let input = input[2..].trim().chars();
        for i in input.into_iter().rev() {
            let i = match i.to_string().parse::<i32>() {
                Ok(i) => i,
                Err(_) => match i {
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => 0,
                },
            };

            res += i * 16_i32.pow(count);
            count += 1;
        }
        print!("{res}");
    } else {
        let mut res = 0;
        let mut count = 0;
        let input = input[1..].trim().chars();
        for i in input.into_iter().rev() {
            let i = i.to_string().parse::<i32>().unwrap();
            res += i * 8_i32.pow(count);
            count += 1;
        }
        print!("{res}");
    }
}
