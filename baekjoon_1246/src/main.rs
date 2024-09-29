use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let total = input.next().unwrap();
    let mut costomer = Vec::with_capacity(input.next().unwrap());

    for _ in 0..costomer.capacity() {
        costomer.push(input.next().unwrap());
    }

    costomer.sort();

    let mut price;
    let mut res_price = 0;
    let mut res_total = 0;

    for i in 0..costomer.len() {
        price = costomer[i];
        let temp = price * (costomer.len() - i).min(total);
        if res_total < temp {
            res_total = temp;
            res_price = price;
        }
    }

    stdout()
        .write_all(format!("{} {}", res_price, res_total).as_bytes())
        .unwrap();
}
