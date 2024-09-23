use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let size = input.next().unwrap().parse::<usize>().unwrap();
    let _count = input.next().unwrap().parse::<usize>().unwrap();

    let mut index = 0;
    let mut wheel = vec!["?"; size];
    let mut hashset = HashSet::with_capacity(size);
    while let (Some(number), Some(ch)) = (input.next(), input.next()) {
        let number = number.parse::<usize>().unwrap();

        index = (index + number) % size;
        if wheel[index] == "?" && hashset.insert(ch) {
            wheel[index] = ch;
        } else if wheel[index] != ch {
            stdout().write_all("!".as_bytes()).unwrap();
            return;
        }
    }

    let mut writer = BufWriter::new(stdout().lock());
    for _ in 0..size {
        writer.write(wheel[index].as_bytes()).unwrap();
        if index == 0 {
            index = size - 1;
        } else {
            index -= 1;
        }
    }
}
