use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let data_count = input.next().unwrap().parse::<usize>().unwrap();
    let ansure_count = input.next().unwrap().parse::<usize>().unwrap();

    let mut data = HashMap::with_capacity(data_count);

    for _ in 0..data_count {
        let site = input.next().unwrap().as_bytes();
        let password = input.next().unwrap().as_bytes();
        data.insert(site, password);
    }

    let mut writer = BufWriter::new(stdout().lock());

    for _ in 0..ansure_count {
        let site = input.next().unwrap().as_bytes();
        writer.write(data.get(site).unwrap()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}
