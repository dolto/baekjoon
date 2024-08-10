use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i64>());

    let category = input.next().unwrap();
    let need_value = input.next().unwrap();

    let mut category_vec = Vec::with_capacity(category as usize);
    for _ in 0..category {
        category_vec.push(input.next().unwrap());
    }

    let mut result = 0;
    let mut count_value = 0;

    for &v in category_vec.iter().rev() {
        let temp = need_value - count_value;
        let multiple = temp / v;
        result += multiple;
        count_value += v * multiple;
        if count_value == need_value {
            break;
        }
    }

    stdout().write_all(format!("{result}").as_bytes()).unwrap();
}
