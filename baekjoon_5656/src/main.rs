use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut output = String::new();
    let mut input = input.split_ascii_whitespace();

    let mut count = 0;

    loop {
        let left_value: i32 = input.next().unwrap().trim().parse().unwrap();
        let command = input.next().unwrap().trim();
        let right_value: i32 = input.next().unwrap().trim().parse().unwrap();

        count += 1;
        let result = match command {
            ">" => {
                if left_value > right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            ">=" => {
                if left_value >= right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            "<" => {
                if left_value < right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            "<=" => {
                if left_value <= right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            "==" => {
                if left_value == right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            "!=" => {
                if left_value != right_value {
                    format!("Case {count}: true\n")
                } else {
                    format!("Case {count}: false\n")
                }
            }
            _ => {
                break;
            }
        };
        output.push_str(&result);
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
